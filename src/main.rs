use bincode::Error;
use crypto::{digest::Digest, sha3::Sha3};

pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

fn main() {
    let data2 = bincode::serialize("dd").unwrap();

    println!("data2:{:?}", &data2);

    let mut hasher = Sha3::sha3_256();
    println!("data2:{:?}", hasher.input(&data2));
    hasher.input(&data2);

    println!("{}", hasher.result_str());
}

/// 증명 해시를 연결할 때 해시를 넣을 쪽에
pub struct MerkleTree {
    pub nodes: Vec<Hash>,
    pub levels: usize,
}
