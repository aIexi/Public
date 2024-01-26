use crate::client::client;
use crate::block::block;
use crate::transaction::transaction;
use crate::blockchain;
use crate::fakeNet::fakeNet;
use std::collections:: HashSet;
use std::rc::Weak;
use std::cell::RefCell;
use crate::eventEmitter::eventEmitter;
use futures::future::{BoxFuture, FutureExt};
use async_trait::async_trait;

#[derive (Clone)]
pub struct miner{
    pub miningRounds: i32,
    pub transactions: HashSet <transaction>,
    pub client: client, 
    pub currentBlock: Option<block>,
    pub pow_target: u64

}

impl miner {
    pub fn new(name: &str, net: Weak<RefCell<fakeNet>>, startingBlock: Option<&block>, keyPair: (String, String), miningRounds: i32, pow_target: u64) -> miner{
        let client = client::new(name, net, startingBlock, keyPair);
        let transactions = HashSet::new();
        miner{miningRounds, transactions, client, currentBlock: None, pow_target}


    }

    pub fn initialize(&mut self) {
        self.startNewSearch(HashSet::new());
        self.onStartMining(3);
    }
    
    pub fn startNewSearch(& mut self, txSet: HashSet<transaction>) {
        let caddr = Some(self.client.address.clone());
        let lb = self.client.lastBlock.clone();
        self.currentBlock = Some(block::new(caddr, lb, self.pow_target, blockchain::COINBASE_AMT_ALLOWED));
        for (tx) in txSet.iter() {
            self.transactions.insert(tx.clone());
        }

        for (tx) in self.transactions.iter() {
            self.currentBlock.as_mut().unwrap().addTransaction(tx, Some(&self.client));
        }
        self.transactions.clear();
        self.currentBlock.as_mut().unwrap().proof = 0;
    }

    pub fn findProof(&mut self, oneAndDone: i32) {
        let pausePoint = self.currentBlock.as_ref().unwrap().proof + self.miningRounds;
        while self.currentBlock.as_ref().unwrap().proof < pausePoint {
            if self.currentBlock.as_ref().unwrap().hasValidProof() {
                self.client.log(&format!("found proof for block {}: {}", self.currentBlock.as_ref().unwrap().chainLength, self.currentBlock.as_ref().unwrap().proof));
                self.announceProof();
                self.receiveBlock(self.currentBlock.as_ref().unwrap().to_owned());
                break;
            }
            self.currentBlock.as_mut().unwrap().proof += 1;
        }
        if oneAndDone > 0 {
            self.onStartMining(oneAndDone - 1);
        }
    }

    pub fn announceProof(& mut self) {
        self.client.net.upgrade().unwrap().borrow_mut().broadcast_proof_found(self.currentBlock.as_ref().unwrap().clone());
    }

    pub fn receiveBlock(& mut self, s: block) -> Option<block>{
        let b = self.client.receiveBlock(s);
        if b.is_none() {
            return None
        }
        
        if self.currentBlock.is_some() && b.as_ref().unwrap().chainLength >= self.currentBlock.as_ref().unwrap().chainLength {
            self.client.log("cutting over to new chain.");
            let txSet = self.syncTransactions(b.as_ref().unwrap().clone());
            self.startNewSearch(txSet);
        }
        
        b

    }

    pub fn syncTransactions(& mut self, mut nb: block) -> HashSet <transaction> {
        let mut cb = self.currentBlock.clone();
        let mut cbTxs = HashSet::new();
        let mut nbTxs = HashSet::new();

        while nb.chainLength > cb.as_ref().unwrap().chainLength {
            for (_, tx) in nb.transactions.iter(){
                nbTxs.insert(tx.clone());
            }
            nb = self.client.blocks.get(&nb.prevBlockHash.unwrap()).unwrap().to_owned();
        
        }

        while cb.is_some() && cb.as_ref().unwrap().id != nb.id {
            for (_, tx) in cb.clone().unwrap().transactions.iter() {
                cbTxs.insert(tx.clone());
            }
            for (_, tx) in nb.transactions.iter() {
                nbTxs.insert(tx.clone());
            }

            cb = self.client.blocks.get(&cb.unwrap().prevBlockHash.unwrap()).map(|b| b.to_owned());
            nb = self.client.blocks.get(&nb.prevBlockHash.unwrap()).unwrap().to_owned();
        }

        for (tx) in nbTxs.iter(){
            cbTxs.remove(&tx.clone());
        }
        cbTxs

    }

    pub fn addTransaction(& mut self, tx: transaction) -> bool {
        self.transactions.insert(tx)
    }

    pub fn postTransaction(& mut self, outputs: Vec<(String, i32)>, fee: i32) -> bool {
        let tx = self.client.postTransaction(outputs, fee);
        self.addTransaction(tx)
    }

}

#[async_trait]
impl eventEmitter for miner{
   
    fn address(&self) -> String {
        self.client.address.clone()
    }

    fn nameOf(&self) -> String {
        self.client.name.clone()
    }

    fn lastBlock(&self) -> Option<block> {
        self.client.lastBlock.clone()
    }

    fn onPostTransaction(&mut self, transaction: transaction){
        self.addTransaction(transaction);
    }

    fn onStartMining(&mut self, oneAndDone: i32){
        self.findProof(oneAndDone);
    }

    fn onProofFound(&mut self, block: block){
        self.client.receiveBlock(block);
    }

    fn onMissingBlock(&mut self, missing: &str, from: &str){
        self.client.provideMissingBlock(missing, from);
    }
}

unsafe impl Send for miner {}
unsafe impl Sync for miner {}