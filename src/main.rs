mod blockchain;
use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.mine_block("Test");

    
}

