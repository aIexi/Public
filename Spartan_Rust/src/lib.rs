use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
pub mod utils;
pub mod block;
pub mod transaction;
pub mod client;
pub mod blockchain;
pub mod eventEmitter;
pub mod fakeNet;
pub mod miner;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
