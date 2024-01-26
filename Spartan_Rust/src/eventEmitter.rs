use crate::transaction::transaction;
use crate::block::block;
use async_trait::async_trait;

#[async_trait]
pub trait eventEmitter{
    fn address(&self) -> String;
    fn nameOf(&self) -> String;
    fn lastBlock(&self) -> Option<block>;
    fn onPostTransaction(&mut self, transaction: transaction);
    fn onStartMining(&mut self, oneAndDone: i32);
    fn onProofFound(&mut self, block: block);
    fn onMissingBlock(&mut self, missing: &str, from: &str);
}