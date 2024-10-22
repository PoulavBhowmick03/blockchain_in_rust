use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;

pub type Result<T> = std::result::Result<T, failure::Error>;
const TARGET_HEXT: usize = 4;

#[derive(Debug, Clone)]
pub struct Block {
    timestamp: u128,
    previous_hash: String,
    hash: String,
    nonce: u128,
    height: usize,
    transactions: String,
    }

#[derive(Debug)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Block {
    pub fn new_block(data:String, previous_hash:String, height:usize) -> Result<Block> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_millis();

        let mut block = Block {
            timestamp: timestamp,
            previous_hash,
            transactions: data,
            height,
            nonce:0,
            hash: String::new(), // Placeholder for hash
        };
        let _ = block.pow();
        Ok(block)
    }
    
    fn pow(&mut self) -> Result<()>{
        info!("Mining the block");
        while !self.validate()? {
            self.nonce +=1;
        }
        let data: Vec<u8> = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        self.hash = hasher.result_str();
        Ok(())
    }

    fn prepare_hash_data(&self)-> Result<Vec<u8>>{
        let content = (
            self.previous_hash.clone(),
            self.timestamp.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce
        );
        let bytes = bincode::serialize(&content)?;
        Ok(bytes)
    }

    fn validate(&self)-> Result<bool>{

        let data: Vec<u8> = self.prepare_hash_data()?;
        let mut hasher = Sha256::new();
        hasher.input(&data[..]);
        let mut vec1: Vec<u8> = vec![];
        vec1.resize(TARGET_HEXT, '0' as u8);
        println!("Target: {:?}", vec1);
        Ok(&hasher.result_str()[..TARGET_HEXT] == String::from_utf8(vec1)?)
    }

    pub fn new_genesis_block() -> Block {
        Block::new_block(String::from("Genesis Block"), String::new(), 0).unwrap()
    }
}

impl Blockchain{
    pub fn new() -> Blockchain{
        Blockchain{
            blocks: vec![Block::new_genesis_block()]
        }
    }

    pub fn add_block(&mut self, data:String) -> Result<()>{
        let prev = self.blocks.last().unwrap();
        let new_block = Block::new_block(data, prev.hash.clone(), prev.height + 1)?;
        self.blocks.push(new_block);
        Ok(())
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_blockchain(){
        let mut chain = Blockchain::new();
        chain.add_block(String::from("Block 1")).unwrap();
        chain.add_block(String::from("Block 2")).unwrap();
        chain.add_block(String::from("Block 3")).unwrap();
        chain.add_block(String::from("Block 4")).unwrap();
        chain.add_block(String::from("Block 5")).unwrap();
        chain.add_block(String::from("Block 6")).unwrap();
        chain.add_block(String::from("Block 7")).unwrap();
        chain.add_block(String::from("Block 8")).unwrap();
        chain.add_block(String::from("Block 9")).unwrap();
        chain.add_block(String::from("Block 10")).unwrap();
        chain.add_block(String::from("Block 11")).unwrap();
        chain.add_block(String::from("Block 12")).unwrap();
        chain.add_block(String::from("Block 13")).unwrap();
        dbg!(chain);
    }
    
}