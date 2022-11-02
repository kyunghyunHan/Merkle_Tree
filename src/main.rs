use crate::error::BlockchainError;
use anyhow::Result;
use serde::{Deserialize, Serialize};
// use sha2::Digest;
mod error;
use crypto::{digest::Digest, sha3::Sha3};
//data값
pub type Data = Vec<u8>;
//hash
pub type Hash = Vec<u8>;
//머클트리 = 이진트리
//머클루트의 용량은 32bytes
//두개씩 묶은 다음 SHA-256암호화 방법을 통해 해시값을 나타내고 그렇게 묶은 값들을 두개씩 묶기를 반복
//머클루트 생성
//거래가 N증가할떄마다 특정 거래를 찾는 경우의 수는 log2(N)으로 늘어난다.
//머클트리는 특정 거래를 찾을떄 효율적

//거래가 1024라면 특정 거래를 찾기 위해 log2(1024 )=10
#[derive(Debug)]
pub struct MerkleTree {
    pub nodes: Vec<Hash>,
    pub levels: usize,
}
//해시 디렉션
/// 증명 해시를 연결할 때 해시를 넣을 쪽에
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HashDirection {
    Left,
    Right,
}
//Proof증명
#[derive(Debug, Default)]
pub struct Proof<'a> {
    /// 증명을 확인할 때 사용할 해시
    /// 튜플의 첫 번째 요소는 연결할 때 해시가 있어야 하는 쪽이다.
    hashes: Vec<(HashDirection, &'a Hash)>,
}
//ERROR
#[derive(Debug)]
pub enum Error {
    CantFindDataInMerkleTree,
    IndexIsNotALeaf,
}

// type Result<T> = std::result::Result<T, Error>;
//머클트리
impl MerkleTree {
    //한단계 위로
    // fn construct_level_up(level: &[Hash]) -> Vec<Hash> {
    //     assert!(is_power_of_two(level.len()));
    //     // 하위 해시를 연결하여 상위 레벨을 찾고 이전 레벨로 이동
    //     //슬라이스의 시작 부분에서 시작하여 한 번에 슬라이스의 chunk_size 요소에 대한 반복자를 반환
    //     level
    //         .chunks(2)
    //         .map(|pair| hash_concat(&pair[0], &pair[1]))
    //         .collect()
    // }

    /// 주어진 입력 데이터로부터 머클 트리 생성
    // pub fn construct(input: &[Data]) -> MerkleTree {
    //     // 일반적으로 주장하는 대신 여기에 결과를 반환하지만
    //     // 제공된 함수 서명으로 유지
    //     assert!(is_power_of_two(input.len()));
    //     // 입력 데이터의 해시를 가져온다. 이것은 Merkle 트리의 잎이 된다.
    //     let mut hashes: Vec<Vec<Hash>> = vec![input.iter().map(hash_data).collect()];
    //     let mut last_level = &hashes[0];

    //     let num_levels = (input.len() as f64).log2() as usize;
    //     // 한 번에 한 레벨씩 트리를 반복하고 다음 레벨에서 노드를 계산한다.

    //     for _ in 0..num_levels {
    //         let mut next_level = vec![MerkleTree::construct_level_up(last_level)];
    //         hashes.append(&mut next_level);
    //         last_level = &hashes[hashes.len() - 1];
    //     }

    //     MerkleTree {
    //         nodes: hashes.into_iter().flatten().collect(),
    //         levels: num_levels + 1,
    //     }
    // }

    /// 주어진 입력 데이터가 주어진 루트 해시를 생성하는지 확인
    // pub fn verify(input: &[Data], root_hash: &Hash) -> bool {
    //     MerkleTree::construct(input).root_hash() == *root_hash
    // }
    /// 머클 트리의 루트 해시를 반환
    pub fn root_hash(&self) -> Hash {
        self.nodes[self.nodes.len() - 1].clone()
    }

    /// Merkle 트리를 구성하는 데 사용된 데이터의 수를 반환
    pub fn num_leaves(&self) -> usize {
        2_usize.pow((self.levels - 1) as u32)
    }

    /// 머클 트리의 levels(기본 데이터의 해시)를 반환
    fn leaves(&self) -> &[Hash] {
        &self.nodes[0..self.num_leaves()]
    }

    /// 주어진 노드 인덱스의 부모 노드 인덱스를 반환
    fn parent_index(&self, index: usize) -> usize {
        // 이 함수는 내부적으로만 사용해야 하므로 여기에서 주장하는 것이 좋다.

        assert!(index != self.nodes.len() - 1, "Root node has no parent");
        assert!(index < self.nodes.len(), "Index outside of tree");

        self.nodes.len() - ((self.nodes.len() - index) / 2)
    }
    /// 주어진 리프 인덱스에 대한 머클 증명을 생성합니다.
    /// 인덱스가 리프에 해당하지 않으면 오류를 반환합니다.
    pub fn get_merkle_proof_by_index(&self, leaf_index: usize) -> Result<Proof> {
        // if leaf_index >= self.num_leaves() {
        //     return Err(Error::IndexIsNotALeaf);
        // }

        let mut proof = Proof::default();
        let mut current_known_index = leaf_index;
        for _ in 0..self.levels - 1 {
            // 이미 한 쪽의 해시를 알고 있거나 이미 계산할 수 있다.
            // 쌍이므로 증명을 위해 다른 하나를 반환해야 한다.
            let corresponding_hash = if current_known_index % 2 == 0 {
                (HashDirection::Right, &self.nodes[current_known_index + 1])
            } else {
                (HashDirection::Left, &self.nodes[current_known_index - 1])
            };
            proof.hashes.push(corresponding_hash);
            // 이제 부모의 해시를 계산할 수 있으므로 부모의
            // 이 노드는 이제 알려진 노드
            current_known_index = self.parent_index(current_known_index);
        }

        Ok(proof)
    }
    //데이터 찾기
    /// 주어진 데이터의 첫 번째 발생에 대한 Merkle 증명을 생성
    /// 머클 트리에서 데이터를 찾을 수 없으면 오류를 반환.
    pub fn get_merkle_proof_by_data(&self, data: &Data) -> i32 {
        // let data_hash = hash_data(data);
        // let leaf_index = self
        //     .leaves()
        //     .iter()
        //     .position(|leaf| *leaf == data_hash)
        // .ok_or(Error::CantFindDataInMerkleTree)?;
        1
        // self.get_merkle_proof_by_index(leaf_index)
    }
}

/// 주어진 증명이 주어진 루트 해시와 데이터에 유효한지 확인
// pub fn verify_merkle_proof(proof: &Proof, data: &Data, root_hash: &Hash) -> bool {
//     let mut current_hash = hash_data(data);

//     for (hash_direction, hash) in proof.hashes.iter() {
//         current_hash = match hash_direction {
//             HashDirection::Left => hash_concat(hash, &current_hash),
//             HashDirection::Right => hash_concat(&current_hash, hash),
//         };
//     }

//     current_hash == *root_hash
// }
//데이터 해시화
// fn hash_data(data: &Data) -> Hash {
//     sha2::Sha256::digest(data).to_vec()
// }

//연결
fn hash_concat(h1: &Hash, h2: &Hash) -> Hash {
    let h3 = h1.iter().chain(h2).copied().collect();
    // hash_data(&h3)
    h3
}

fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}
//
pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: Serialize + ?Sized,
{
    //Bincode는 작은 바이너리 직렬화 전략을 사용하여 인코딩 및 디코딩하기 위한 상자
    //serialize:기본 구성을 사용하여 직렬화 가능한 개체를 Vec바이트 단위로 직렬화
    Ok(bincode::serialize(data)?)
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    id: String,
    vin: String,
    vout: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Block {
    hash: String,
}
//트랜잭션 hash
pub fn set_txs_hash(txs: &[Transaction]) -> String {
    let txs_ser = serialize(txs);
    match txs_ser {
        Ok(txs_ser) => {
            let hashs = hash_to_str(&txs_ser);
            hashs
        }
        Err(e) => {
            println!("err");
            "error".to_string()
        }
    }
}
//
pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    // 입력 메시지
    hasher.input(data);
    //해시 다이제스트 읽기
    hasher.result_str()
}

//test
fn main() {
    let test_data = [Transaction {
        id: "1".to_string(),
        vin: "2".to_string(),
        vout: "3".to_string(),
    }];
    let test_data2 = [Transaction {
        id: "1".to_string(),
        vin: "2".to_string(),
        vout: "3".to_string(),
    }];
    let tett = set_txs_hash(&test_data);

    println!("{}", tett);

    let txs_ser = serialize(&test_data);

    let txs_ser2 = serialize(&test_data2);

    let ddd = bincode::serialize(&test_data2).unwrap();
    println!("ddd:{:?}", ddd);
    let con = hash_concat(&ddd, &ddd);
    println!("{:?}", con);
}
//TDD
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let test_data = 1;

        assert_eq!(test_data, 1);
    }
    //트랜잭션 해시 되는지
    #[test]
    fn test_transcion() {
        let test_data = [Transaction {
            id: "1".to_string(),
            vin: "2".to_string(),
            vout: "3".to_string(),
        }];

        assert_eq!(
            set_txs_hash(&test_data),
            "0be44d4de480f8ff39719fa36f229bf09268ceaea192c3a5e7767a58639817b1"
        );
    }
}
