use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("First block after genesis".to_string());
    blockchain.add_block("Second block after genesis".to_string());
    blockchain.add_block("Third block after genesis".to_string());

    println!("{:?}", blockchain);

    println!("Is blockchain valid? {}", blockchain.is_chain_valid());
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash: previous_hash.clone(),
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.index.to_string().as_bytes());
        hasher.update(self.timestamp.to_string().as_bytes());
        hasher.update(self.data.as_bytes());
        hasher.update(self.previous_hash.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        let mut blockchain = Blockchain { chain: Vec::new() };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        self.chain.push(genesis_block);
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap().clone();
        let new_block = Block::new(previous_block.index + 1, data, previous_block.hash);
        self.chain.push(new_block);
    }

    fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }
}
