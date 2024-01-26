use std::collections:: HashMap;
use serde::{Serialize, Deserialize};
use serde::ser::{Serializer, SerializeStruct};
use crate::utils;
use serde_json;

#[derive(Clone, Hash, Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct transaction{
    pub from: String, 
    pub nonce: i32,
    pub pub_key: String, 
    pub sig: Option <String>,
    pub fee: i32,
    pub outputs: Vec <(String, i32)>, 
    pub data: String,
    pub id: String

}

impl transaction{
    pub fn new(from: &str, nonce: i32, pub_key: &str, sig: Option <String>, fee: i32, outputs: Vec <(String, i32)>, data: &str) -> transaction{
        //let id = utils::hash(&utils::generateKeypair().0);
        let mut tx = transaction{from:from.to_owned(), nonce, pub_key:pub_key.to_owned(), sig, fee, outputs, data:data.to_owned(), id: "".to_string()};
        let id = utils::hash(&format!("TX {}", &serde_json::to_string(&tx).unwrap()));
        tx.id = id;
        tx
    }

    /*pub fn id(&self) -> String {
        utils::hash(&format!("TX {}", &serde_json::to_string(&self).unwrap()))
    }*/

    pub fn sign(&mut self, priv_key: &str) {
        self.sig = Some(utils::sign(priv_key, &self.id))
    }
    
    pub fn validSignature(&self) -> bool{
        !self.sig.is_none() && 
        utils::addressMatchesKey(&self.from, &self.pub_key) && utils::verifySignature(&self.pub_key, &self.id, &self.sig.as_ref().unwrap())
    }

    pub fn sufficientFunds(&self, balances: &HashMap <String, i32>) -> bool{
        self.totalOutput() <= *balances.get(&self.from).unwrap()
    }

    pub fn totalOutput(&self) -> i32{
        self.outputs.iter().fold(self.fee, |sum, (_, amt)| sum + amt)
    }
}

unsafe impl Send for transaction {}
unsafe impl Sync for transaction {}