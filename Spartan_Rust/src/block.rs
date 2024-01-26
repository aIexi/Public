use std::time::{Duration, Instant};
use std::collections:: HashMap;
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeStruct};
use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use crate::utils;
use crate::transaction::transaction;
use crate::client::client;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct block{
    pub prevBlockHash: Option<String>, 
    pub target: u64,
    pub balances: HashMap <String, i32>, 
    pub nextNonce: HashMap <String, i32>,
    pub transactions: HashMap <String, transaction>,
    pub chainLength: i32, 
    pub timestamp: i64,
    pub rewardAddr: Option<String>,
    pub coinbaseReward: i32,
    pub proof: i32,
    pub id: String

}

impl block {
    pub fn new(rewardAddr: Option<String>, prevBlock: Option<block>, target: u64, coinbaseReward: i32) -> block{
        let prevBlockHash = prevBlock.clone().map(|pb| pb.id);
        let mut balances = prevBlock.clone().map_or(HashMap::new(), |pb| pb.balances);
        let nextNonce = prevBlock.clone().map_or(HashMap::new(), |pb| pb.nextNonce);
        let mut winnerBalance = 0;
        if prevBlock.is_some() && prevBlock.clone().unwrap().rewardAddr.is_some() {
            winnerBalance = *prevBlock.clone().unwrap().balances.get(&prevBlock.clone().unwrap().rewardAddr.unwrap()).unwrap_or(&0);
            *balances.entry(prevBlock.clone().unwrap().rewardAddr.unwrap()).or_insert(0) += winnerBalance + prevBlock.clone().unwrap().totalRewards();
        }
        let transactions = HashMap::new();
        let chainLength = prevBlock.map_or(0, |pb| pb.chainLength + 1);
        let timestamp = chrono::offset::Utc::now().timestamp();
        let mut b = block{prevBlockHash, target, balances, nextNonce, transactions, chainLength, timestamp, rewardAddr, coinbaseReward, proof: 0, id: "".to_string()};
        let id = utils::hash(&format!("blk {}", &b.serialize()));
        b.id = id;
        b



    }
    
    
    pub fn isGenesisBlock(&self) -> bool{
        self.chainLength == 0
    }

    pub fn hasValidProof(&self) -> bool{
        let mut h = utils::hash(&self.serialize()); 
        h.truncate(10);
        let n = u64::from_str_radix(&h, 16).unwrap();
        let r = n < self.target;
        r
    }

    pub fn serialize(&self) -> String{
        serde_json::to_string(&self).unwrap()
    }

    /*pub fn id(&self) -> String {
        utils::hash(&format!("blk {}", &self.serialize()))
    }*/
    

    /*pub fn hashVal(&self) -> String{
        utils::hash(&self.serialize())
    }*/

    pub fn addTransaction(&mut self, tx: &transaction, client: Option<&client>) -> bool{
        if self.transactions.get(&tx.id).is_some(){
            if let Some(client) = client {
                client.log(&format!("Duplicate transaction {}.", tx.id));
            }
            return false;
        }
        else if tx.sig.is_none(){
            if let Some(client) = client {
                client.log(&format!("Unsigned transaction {}.", tx.id));
            }
            return false;
        }
        else if !tx.validSignature(){
            if let Some(client) = client {
                client.log(&format!("Invalid signature for transaction {}.", tx.id));
            }
            return false;
        }
        else if !tx.sufficientFunds(&self.balances){
            if let Some(client) = client {
                client.log(&format!("Insufficient gold for transaction {}.", tx.id));
            }
            return false;
        }
        
        let nonce = *self.nextNonce.get(&tx.from).unwrap_or(&0);
        if tx.nonce < nonce {
            if let Some(client) = client {
                client.log(&format!("Replayed transaction {}.", tx.id));
            }
            return false;
        }
        else if tx.nonce > nonce{
            if let Some(client) = client {
                client.log(&format!("Out of order transaction {}.", tx.id));
            }
            return false;
        }
        else {
            self.nextNonce.insert(tx.from.to_owned(), nonce + 1);
        }
        self.transactions.insert(tx.id.clone(), tx.clone());
        let senderBalance = self.balanceOf(&tx.from);
        self.balances.insert(tx.from.to_owned(), senderBalance - tx.totalOutput());
        for (addr, amt) in tx.outputs.iter(){
            self.balances.insert(addr.to_owned(), amt + self.balanceOf(&addr));
        }
        true

    }

    pub fn rerun(&mut self, prevBlock: &block) -> bool{
        self.balances = prevBlock.balances.clone();
        self.nextNonce = prevBlock.nextNonce.clone();
        if let Some(ref addr) = prevBlock.rewardAddr{
            let winnerBalance = self.balanceOf(&addr);
            *self.balances.entry(addr.to_owned()).or_insert(0) += winnerBalance + prevBlock.totalRewards();
        }
        let txs = self.transactions.clone();
        self.transactions = HashMap::new();
        for (_, v) in txs.into_iter(){
            let b = self.addTransaction(&v, None);
            if(!b){
                return false;
            }
        }
        true
    }

    pub fn balanceOf(&self, addr: &str) -> i32{
        *self.balances.get(addr).unwrap_or(&0)
    }

    pub fn totalRewards(&self) -> i32{
        self.transactions.iter().fold(self.coinbaseReward, |sum, (_, tx)| sum + tx.fee)
    }

    pub fn contains(&self, tx: &str) -> bool{
        self.transactions.contains_key(tx)
    }


}

unsafe impl Send for block {}
unsafe impl Sync for block {}