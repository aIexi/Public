use std::time::Duration;
use std::thread::sleep;
use std::collections::HashMap;
use crate::eventEmitter::eventEmitter;
use crate::transaction::transaction;
use crate::block::block;
use std::rc::Rc;
use std::cell::RefCell;
use tokio_js_set_interval::set_timeout;

pub struct fakeNet{
    pub clients: HashMap<String, Rc<RefCell<dyn eventEmitter>>>,
    pub chanceMessageFails: i32,
    pub messageDelayMax: u64
}

impl fakeNet{
    pub fn new(chanceMessageFails: i32, messageDelay: u64) -> fakeNet {
        fakeNet {clients: HashMap::new(), chanceMessageFails, messageDelayMax: messageDelay}
    }

    pub fn register(&mut self, clientList: Vec<Rc<RefCell<dyn eventEmitter>>>){
        for client in clientList.into_iter() {
            let addr = client.borrow().address();
            self.clients.insert(addr, client);
        }
            
    }

    pub fn broadcast_post_transaction(&mut self, transaction: transaction){
        for (address, client) in self.clients.iter_mut(){
            //sleep(Duration::new(self.messageDelayMax, 0));
            if let Ok(mut c) = client.try_borrow_mut() {
                c.onPostTransaction(transaction.clone());
            }
        }
    }

    pub fn broadcast_start_mining(&mut self, oneAndDone: i32){
        for (address, client) in self.clients.iter_mut(){
            sleep(Duration::new(self.messageDelayMax, 0));
            if let Ok(mut c) = client.try_borrow_mut() {
                c.onStartMining(oneAndDone);
            }
        }
    }

    pub fn broadcast_proof_found(&mut self, block: block){
        for (address, client) in self.clients.iter_mut(){
            sleep(Duration::new(self.messageDelayMax, 0));
            if let Ok(mut c) = client.try_borrow_mut() {
                c.onProofFound(block.clone());
            }
        }
    }

    pub fn broadcast_on_missing_block(&mut self, missing: &str, from: &str){
        for (address, client) in self.clients.iter_mut(){
            sleep(Duration::new(self.messageDelayMax, 0));
            if let Ok(mut c) = client.try_borrow_mut() {
                c.onMissingBlock(missing, from);
            }
        }
    }

    pub fn send_message_proof_found(&mut self, address: &str, block: block){
        if let Some(client) = self.clients.get_mut(address) {
            sleep(Duration::new(self.messageDelayMax, 0));
            client.borrow_mut().onProofFound(block);
        }
    }
}

unsafe impl Send for fakeNet {}
unsafe impl Sync for fakeNet {}