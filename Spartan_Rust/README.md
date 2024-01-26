# Spartan Rust

Spartan Rust is a small proof-of-work (POW) blockchain cryptocurrency.
It is a partial Rust implementation of: https://github.com/taustin/spartan-gold




# Add as dependency

In your Cargo.toml: 
```
[dependencies]
cs168_final_project = { path = "../cs168_final_project" }
```

# Example

```
// imports elided
fn main() {
    println!("Starting simulation.  This may take a moment...");
    let target = blockchain::POW_BASE_TARGET >> 2;
    let mut strongNet = Rc::new(RefCell::new(fakeNet::new(1, 1)));
    let mut net = Rc::downgrade(& strongNet);
    let mut alice = Rc::new(RefCell::new(client::new("Alice", net.clone(), None, utils::generateKeypair())));
    let mut bob = Rc::new(RefCell::new(client::new("Bob", net.clone (), None, utils::generateKeypair())));
    let cbm = vec![(alice.borrow().address.clone(), 233), (bob.borrow().address.clone(), 99)].into_iter().collect::<HashMap <_, _>>();
    let mut genesis = blockchain::blockchain::makeGenesis(5, 25, 1, 6, Some(cbm), None);
    alice.borrow_mut().setGenesisBlock(genesis.genesisBlock.clone());
    bob.borrow_mut().setGenesisBlock(genesis.genesisBlock.clone());
    
    println!("Initial balances:");
    println!("Alice has {} gold.", client.lastBlock.as_ref().unwrap().balanceOf(alice));
    println!("Bob has {} gold.", client.lastBlock.as_ref().unwrap().balanceOf(bob));
    strongNet.borrow_mut().register(vec![alice.clone(), bob.clone()]);

    println!("Alice is transferring 40 gold to {}.", bob.borrow().address);
    alice.borrow_mut().postTransaction(vec![(bob.borrow().address.clone(), 40)], 0);

    println!("Alice has {} gold.", client.lastBlock.as_ref().unwrap().balanceOf(alice));
    println!("Bob has {} gold.", client.lastBlock.as_ref().unwrap().balanceOf(bob));
}
```