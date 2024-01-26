use std::collections:: HashMap;
use crate::eventEmitter::eventEmitter;
use crate::fakeNet::fakeNet;
use std::rc::Weak;
use crate::block::block;
use crate::transaction::transaction;
use crate::utils;
use crate::blockchain;
use std::cell::RefCell;
use async_trait::async_trait;

pub struct txData{
    pub sig: Option <String>,
    pub fee: i32,
    pub outputs: Vec <(String, i32)>, 
    pub data: String,

}

#[derive (Clone)]
pub struct client{
    pub net: Weak<RefCell<fakeNet>>, 
    pub name: String,
    pub keyPair: (String, String), 
    pub address: String,
    pub nonce: i32,
    pub pendingOutgoingTransactions: HashMap <String, transaction>, 
    pub pendingReceivedTransactions: HashMap <String, transaction>,
    pub blocks: HashMap <String, block>,
    pub pendingBlocks: HashMap <String, Vec<block>>,
    pub lastConfirmedBlock: Option<block>,
    pub lastBlock: Option<block>

}

impl client {
    pub fn new(name: &str, net: Weak<RefCell<fakeNet>>, startingBlock: Option<&block>, keyPair: (String, String)) -> client{
        let address = utils::calcAddress(&keyPair.1);
        let nonce = 0;
        let pendingOutgoingTransactions = HashMap::new();
        let pendingReceivedTransactions = HashMap::new();
        let mut blocks = HashMap::new();
        let pendingBlocks = HashMap::new();
        let mut lastConfirmedBlock = None;
        let mut lastBlock = None;
        if let Some(b) = startingBlock{
            lastConfirmedBlock = Some(b.clone());
            lastBlock = Some(b.clone());
            blocks.insert(b.id.clone(), b.clone());

        }
        let name = name.to_owned();
        client{net, name, keyPair, address, nonce, pendingOutgoingTransactions, pendingReceivedTransactions, blocks, pendingBlocks, lastConfirmedBlock, lastBlock}
    }

    pub fn log(&self, msg: &str){
        println!("{}: {}", self.name, msg);
    }

    pub fn setGenesisBlock(&mut self, startingBlock: block){
        if self.lastBlock.is_some(){
            panic!("Cannot set genesis block for existing blockchain.");
        }
        self.lastConfirmedBlock = Some(startingBlock.clone());
        self.lastBlock = Some(startingBlock.clone());
        self.blocks.insert(startingBlock.id.clone(), startingBlock.clone());
    }

    pub fn confirmedBalance(&self) -> i32 {
        self.lastConfirmedBlock.clone().unwrap().balanceOf(&self.address)
    }


    pub fn availableGold(&self) -> i32 {
        let pendingSpent = self.pendingOutgoingTransactions.iter().fold(0, |sum, (_, tx)| sum + tx.totalOutput());
        self.confirmedBalance() - pendingSpent
    }


    pub fn postTransaction(&mut self, outputs: Vec<(String, i32)>, fee: i32) -> transaction {
        let totalPayments = outputs.iter().fold(0, |sum, (addr, amt)| sum + amt) + fee;
        if totalPayments > self.availableGold(){
            panic!("Requested {}, but account only has {}", totalPayments, self.availableGold());
        }
        self.postGenericTransaction(
            txData{sig:None, fee, outputs, data: String::from("")}
        )

    }

    pub fn postGenericTransaction(&mut self, data: txData) -> transaction {
        let mut tx = transaction::new(
            &self.address, self.nonce, &self.keyPair.1, data.sig, data.fee, data.outputs, &data.data
        );
        tx.sign(&self.keyPair.0); 
        self.pendingOutgoingTransactions.insert(tx.id.clone(), tx.clone());
        self.nonce += 1;
        self.net.upgrade().unwrap().borrow_mut().broadcast_post_transaction(tx.clone());
        tx

    }


    pub fn receiveBlock(&mut self, mut blockN: block) -> Option<block>{
        if self.blocks.contains_key(&blockN.id) {
            return None;
        }
        if !blockN.hasValidProof() && !blockN.isGenesisBlock(){
            self.log(&format!("{} does not have a valid proof.", blockN.id));
            return None;
        }
        let prevBlock = self.blocks.get(&blockN.clone().prevBlockHash.unwrap());
        if prevBlock.is_none() && !blockN.clone().isGenesisBlock(){
            let mut stuckBlocks = self.pendingBlocks.get(&blockN.clone().prevBlockHash.unwrap()).map_or_else(|| {
                self.requestMissingBlock(&blockN.clone().prevBlockHash.unwrap());
                vec![]
            }, |v| v.clone());
            
            stuckBlocks.push(blockN.clone());
            self.pendingBlocks.insert(blockN.clone().prevBlockHash.unwrap(), stuckBlocks.to_owned());
            return None;
        }
        if !blockN.isGenesisBlock(){
            let success = blockN.rerun(prevBlock.unwrap());
            if !success {
                return None;
            }
        }

        self.blocks.insert(blockN.id.clone(), blockN.clone());

        if self.lastBlock.clone().unwrap().chainLength < blockN.chainLength {
            self.lastBlock = Some(blockN.clone());
            self.setLastConfirmed();
        }

        let unstuckBlocks = self.pendingBlocks.get(&blockN.id).map_or(vec![], |v| v.clone());
        self.pendingBlocks.remove(&blockN.id);
        unstuckBlocks.iter().for_each(|b| {
            self.log(&format!("Processing unstuck block {}", b.id));
            self.receiveBlock(b.clone());
        } );
        Some(blockN.clone())

    }


    pub fn requestMissingBlock(&self, prevBlockHash: &str){
        self.log(&format!("Asking for missing block: {}", prevBlockHash));
        //self.net.upgrade().unwrap().borrow_mut().broadcast_on_missing_block(prevBlockHash, &self.address);
        //loop {
            if let Ok(mut n) = self.net.upgrade().unwrap().try_borrow_mut(){
                n.broadcast_on_missing_block(prevBlockHash, &self.address);
                //break;
            }
        //}
    }

    pub fn resendPendingTransaction(&self){
        for (_, tx) in self.pendingOutgoingTransactions.iter(){
            self.net.upgrade().unwrap().borrow_mut().broadcast_post_transaction(tx.clone());
        }
    }


    pub fn provideMissingBlock(&self, missing: &str, from: &str){
        if let Some(block) = self.blocks.get(missing) {
            self.log(&format!("Providing mossing block {}", missing));
            self.net.upgrade().unwrap().borrow_mut().send_message_proof_found(from, block.clone());
        }
        
    }

    pub fn setLastConfirmed(&mut self){
        if let Some(mut block) = self.lastBlock.clone(){
            let mut confirmedBlockHeight = block.chainLength - blockchain::CONFIRMED_DEPTH;
            if (confirmedBlockHeight < 0) {
                confirmedBlockHeight = 0;
            }
            while (block.chainLength > confirmedBlockHeight) {
                block = self.blocks.get(&block.prevBlockHash.unwrap()).unwrap().clone();
            }
            self.lastConfirmedBlock = Some(block);
            let pot = self.pendingOutgoingTransactions.iter().map(|(id, tx)| (id.clone(), tx.clone())).collect::<Vec <_>>();
            for (txID, tx) in pot.iter() {
                if self.lastConfirmedBlock.clone().unwrap().transactions.get(&txID.clone()).is_some() {
                    self.pendingOutgoingTransactions.remove(&txID.clone());
                }
            } 

        }
        
    }

    pub fn showAllBalances(&self){
        if let Some(ref lcb) = self.lastConfirmedBlock {
            println!("Showing balances:");
            for (id, balance) in lcb.balances.iter(){
                println!("    {}: {}", id, balance);
            }
        }
    }

    pub fn showBlockchain(&self){
        let mut block = (&self.lastBlock).as_ref();
        self.log(&format!("blockchain:"));
        while let Some(b) = block {
            println!("{}", b.id);
            block = self.blocks.get(&b.prevBlockHash.clone().unwrap());
        }
    }
}    

#[async_trait]
impl eventEmitter for client{
    
    fn address(&self) -> String {
        self.address.clone()
    }

    fn nameOf(&self) -> String {
        self.name.clone()
    }

    fn lastBlock(&self) -> Option<block> {
        self.lastBlock.clone()
    }

    fn onPostTransaction(&mut self, _transaction: transaction){
        {}
    }
    fn onStartMining(&mut self, _: i32){
        {}
    }
    fn onProofFound(&mut self, block: block){
        self.receiveBlock(block);
    }
    fn onMissingBlock(&mut self, missing: &str, from: &str){
        self.provideMissingBlock(missing, from);
    }
}

unsafe impl Send for client {}
unsafe impl Sync for client {}