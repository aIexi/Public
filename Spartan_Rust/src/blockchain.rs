use std::collections::HashMap;
use crate::block::block;
use crate::client::client;

pub const MISSING_BLOCK: &'static str = "MISSING_BLOCK";
pub const POST_TRANSACTION: &'static str = "POST_TRANSACTION";
pub const PROOF_FOUND: &'static str = "PROOF_FOUND";
pub const START_MINING: &'static str = "START_MINING";
pub const NUM_ROUNDS_MINING: i32 = 2000;
pub const POW_BASE_TARGET: u64 = 1099511627775;
pub const POW_LEADING_ZEROES: i32 = 15;
pub const COINBASE_AMT_ALLOWED: i32 = 25;
pub const DEFAULT_TX_FEE: i32 = 1;
pub const CONFIRMED_DEPTH: i32 = 6;


pub struct blockchain{
    pub powTarget: u64,
    pub coinbaseAmount: i32,
    pub defaultTxFee: i32,
    pub confirmedDepth: i32,
    pub genesisBlock: block


}

impl blockchain {
    pub fn makeGenesis(powLeadingZeroes: u64, coinbaseAmount: i32, defaultTxFee: i32, confirmedDepth: i32, 
        clientBalanceMap: Option<HashMap<String, i32>>, startingBalances: Option<HashMap<String, i32>> ) -> blockchain {
    
        if clientBalanceMap.is_some() && startingBalances.is_some() {
            panic!("You may set clientBalanceMap OR set startingBalances, but not both.");
        }

        let powTarget = POW_BASE_TARGET >> powLeadingZeroes;

        let mut balances = startingBalances.unwrap_or(HashMap::new());

        if let Some(ref cbm) = clientBalanceMap {
            for (client, balance) in cbm.iter() {
                balances.insert(client.clone(), *balance);
            }
        }

        let mut g = block::new(None, None, powTarget, COINBASE_AMT_ALLOWED);

        for (addr, amt) in balances.iter(){
            g.balances.insert(addr.to_owned(), *amt);
        }

        /*if let Some(mut cbm) = clientBalanceMap {
            for (client, _) in cbm.iter_mut() {
                client.setGenesisBlock(g.clone());
            }
        }*/

        blockchain {powTarget, coinbaseAmount, defaultTxFee, confirmedDepth, genesisBlock: g} 

    }
}
