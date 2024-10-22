mod block;

use block::{Blockchain};

fn main() {
    let mut chain = Blockchain::new();
    
    chain.add_block(String::from("Block 1")).unwrap();
    chain.add_block(String::from("Block 2")).unwrap();
    chain.add_block(String::from("Block 3")).unwrap();
    chain.add_block(String::from("Block 4")).unwrap();
    chain.add_block(String::from("Block 5")).unwrap();
    
    dbg!(&chain);
}