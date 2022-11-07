use crate::error::BlockchainError;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
mod error;
use crypto::{digest::Digest, sha3::Sha3};
pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

/*
트랜잭션
value
to
from
data
id: 트랜잭션 해시 값
vin: 트랜잭션 입력 세트
vout: 트랜잭션 출력 수집
*/

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    id: String,
    vin: String,
    vout: String,
}
/*
머클트리
머클트리 = 이진트리
머클루트의 용량은 32bytes
두개씩 묶은 다음 SHA-256암호화 방법을 통해 해시값을 나타내고 그렇게 묶은 값들을 두개씩 묶기를 반복
거래가 N증가할떄마다 특정 거래를 찾는 경우의 수는 log2(N)으로 늘어난다.
머클트리는 특정 거래를 찾을떄 효율적
거래가 1024라면 특정 거래를 찾기 위해 log2(1024 )=10
MerkleRoot:최종 결과 해시값
맨아래행의 해시를 잎
중간해시를 가지
맨위에 해시를 루트
*/
#[derive(Debug)]
pub struct MerkleRoot {
    pub hash: String,
}
#[derive(Debug)]
pub struct MerkleTree {
    pub nodes: Vec<Hash>,
    pub levels: usize,
    pub merkle_root: MerkleRoot,
}
/*

해시 디렉션
증명 해시를 연결할 때 해시를 넣을 쪽에
*/

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HashDirection {
    Left,
    Right,
}

/*
Proof증명
*/
#[derive(Debug, Default)]
pub struct Proof<'a> {
    /// 증명을 확인할 때 사용할 해시
    /// 튜플의 첫 번째 요소는 연결할 때 해시가 있어야 하는 쪽
    hashes: Vec<(HashDirection, &'a Hash)>,
}

/*
머클트리

1.트랜잭션 직렬화
2.트랜잭션 해시
3.트랜잭션 직렬화
4.트랜잭션 합치고 해시
5.트랜잭션 직렬화

순서
트랜잭션생성
트랜잭선 해시화
트랜잭션 합치고
합침과 동시에 level 올라가고
함치고 해시


*/
impl MerkleTree {
    //트랜잭션들을 받아서 모임일 받아서
    pub fn mekle_tree_return(datas: Vec<Vec<u8>>) -> MerkleRoot {
        println!("트랜잭션들{:?}", &datas);
        //확인하고
        assert!(is_power_of_two(datas.len()));
        println!("이거{}", is_power_of_two(datas.len()));
        let num_levels = (datas.len() as f64).log2() as usize;
        //트랜잭션들을 받아서 직렬화
        let mut hashes: Vec<Vec<Hash>> = vec![datas.iter().map(hash_data).collect()];
        let mut last_level = &hashes[0];
        println!("몇번인지확인:{}", num_levels);
        println!("hashe집합:{:?}", hashes);
        println!("마지막해시 last_level:{:?}", last_level.len());

        for _ in 0..num_levels {
            let mut next_level = vec![MerkleTree::construct_level_up(last_level)];
            hashes.append(&mut next_level);
            last_level = &hashes[hashes.len() - 1];
        }
        println!("last_level{:?}", last_level[0]);
        //into_iter:소유권을 가져감
        //flatten:중첩된 구조를 평면화
        let test = hash_to_str(&last_level[0]);
        println!("{}", test);
        println!("{}", test);
        let mekle_tree = MerkleTree {
            nodes: hashes.into_iter().flatten().collect(),
            levels: num_levels + 1,
            merkle_root: MerkleRoot { hash: test.clone() },
        };

        MerkleRoot { hash: test.clone() }
    }

    //트랙잭션 hash
    pub fn transaction_hash(data: &Transaction) -> String {
        let txs_ser = serialize(data);
        match txs_ser {
            Ok(txs_ser) => {
                let hashs = hash_to_str(&txs_ser);
                hashs
            }
            Err(e) => {
                println!("err");
                assert!(false);
                "error".to_string()
            }
        }
    }
    //머클트리 체인 연결

    fn hash_concat(h1: &Hash, h2: &Hash) -> Hash {
        //반복자 체인
        //두개의 반복자를 가져와서 둘모두에 대한  새로운 반복자 생성
        let h3 = h1.iter().chain(h2).copied().collect();
        hash_data(&h3)
    }
    //한단계 위로
    /*
    해시집합 받아서
    하위해시를 연결
    */
    fn construct_level_up(level: &[Hash]) -> Vec<Hash> {
        assert!(is_power_of_two(level.len()));
        // 하위 해시를 연결하여 상위 레벨을 찾고 이전 레벨로 이동
        //슬라이스의 시작 부분에서 시작하여 한 번에 슬라이스의 chunk_size 요소에 대한 반복자를 반환
        level
            .chunks(2)
            .map(|pair| Self::hash_concat(&pair[0], &pair[1]))
            .collect()
    }
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

    /*
    주어진 리프 인덱스에 대한 머클 증명을 생성합니다.
    인덱스가 리프에 해당하지 않으면 오류를 반환합니다.
    */
    pub fn get_merkle_proof_by_index(&self, leaf_index: usize) -> Result<Proof> {
        if leaf_index >= self.num_leaves() {
            return Err(Error::msg("message"));
        }
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
    /*
     데이터 찾기
     주어진 데이터의 첫 번째 발생에 대한 Merkle 증명을 생성
    머클 트리에서 데이터를 찾을 수 없으면 오류를 반환.
    */
    pub fn get_merkle_proof_by_data(&self, data: &Data) -> Result<Proof> {
        let data_hash = hash_data(data);

        //position:반복기에서 요소를 검색하여 해당 index 몇번쨰인지 반환
        let leaf_index = self
            .leaves()
            .iter()
            .position(|leaf| *leaf == data_hash)
            .ok_or(Error::msg("message"))?;

        self.get_merkle_proof_by_index(leaf_index)
    }
}

/// 주어진 증명이 주어진 루트 해시와 데이터에 유효한지 확인
pub fn verify_merkle_proof(proof: &Proof, data: &Data, root_hash: &Hash) -> bool {
    let mut current_hash = hash_data(data);

    for (hash_direction, hash) in proof.hashes.iter() {
        current_hash = match hash_direction {
            HashDirection::Left => MerkleTree::hash_concat(hash, &current_hash),
            HashDirection::Right => MerkleTree::hash_concat(&current_hash, hash),
        };
    }

    current_hash == *root_hash
}
//데이터 직렬화
fn hash_data(data: &Data) -> Hash {
    let serialize_transaction3 = bincode::serialize(&data).unwrap();
    serialize_transaction3
}

fn is_power_of_two(n: usize) -> bool {
    //4==0 4 &4 -4-1 ==0
    //비트연산자
    //0이 아니거나 n & (n-1) 이 0이면true
    n != 0 && (n & (n - 1)) == 0
}
//직렬화
pub fn serialize<T>(data: &T) -> Result<Vec<u8>, BlockchainError>
where
    T: Serialize + ?Sized,
{
    //Bincode는 작은 바이너리 직렬화 전략을 사용하여 인코딩 및 디코딩하기 위한 상자
    //serialize:기본 구성을 사용하여 직렬화 가능한 개체를 Vec바이트 단위로 직렬화
    Ok(bincode::serialize(data)?)
}

//트랜잭션 hash
pub fn set_txs_hash(txs: &[String]) -> String {
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

//해시로 변경
pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    // 입력 메시지
    hasher.input(data);
    //해시 다이제스트 읽기
    hasher.result_str()
}

//test
fn main() {
    assert!(true);
    //트랜잭션데이터
    let tx1 = Transaction {
        id: "1".to_string(),
        vin: "2".to_string(),
        vout: "3".to_string(),
    };
    let tx2 = Transaction {
        id: "4".to_string(),
        vin: "5".to_string(),
        vout: "6".to_string(),
    };
    //트랜잭션 해시 및 직렬화
    let hash_tx1 = MerkleTree::transaction_hash(&tx1);
    let hash_tx2 = MerkleTree::transaction_hash(&tx2);
    println!("트랜잭션 해시 및 직렬화:{:?}", hash_tx1);

    //해시 집합 및 직렬화
    let mut txs = Vec::new();
    txs.push(bincode::serialize(&hash_tx1).unwrap());
    txs.push(bincode::serialize(&hash_tx2).unwrap());
    txs.push(bincode::serialize(&hash_tx1).unwrap());
    txs.push(bincode::serialize(&hash_tx2).unwrap());

    println!("해시 집합:{:?}", txs);

    //연결
    let concat = MerkleTree::hash_concat(&txs[0], &txs[1]);
    println!("연결{:?}", concat);
    // // let tss = MerkleTree::construct_level_up(&txs);

    //연결한 해시들 해시
    let hash = hash_to_str(&concat);
    println!("연결한 해시들 해시:{:?}", hash);
    //해시를 다시 직렬
    let ser_hash1 = bincode::serialize(&hash).unwrap();
    println!("해시를 다시 직렬 {:?}", ser_hash1);
    let ser_hash2 = bincode::serialize(&hash).unwrap();
    println!("해시를 다시 직렬 {:?}", ser_hash2);

    //역직렬화
    let deserialize_hash: String = bincode::deserialize(&ser_hash1).unwrap();
    println!("{:?}", deserialize_hash);

    let mekle_root = MerkleTree::mekle_tree_return(txs).hash;
    println!("merkle_root:{:?}", mekle_root);

    let test = [0];
    let test_hash1 = bincode::serialize(&test).unwrap();
    let test2 = 0;
    let test_hash2 = bincode::serialize(&test2).unwrap();

    println!("1{:?}", test_hash1);
    println!("2{:?}", test_hash2);

    // let ara = MerkleTree::construct_level_up(&txs);
}

//TDD
#[cfg(test)]
mod tests {
    use super::*;
    /*트랜잭션 해시되는지 */
    #[test]
    fn test1() {
        let tx1 = Transaction {
            id: "1".to_string(),
            vin: "2".to_string(),
            vout: "3".to_string(),
        };
        let tx2 = Transaction {
            id: "4".to_string(),
            vin: "5".to_string(),
            vout: "6".to_string(),
        };
        //트랜잭션 해시 및 직렬화
        let hash_tx1 = MerkleTree::transaction_hash(&tx1);
        let hash_tx2 = MerkleTree::transaction_hash(&tx2);
        println!("트랜잭션 해시 및 직렬화:{:?}", hash_tx1);

        assert!(hash_tx1 != hash_tx2);
    }

    #[test]
    fn test2() {
        assert!(1 == 1);
    }
    #[test]
    fn test3() {
        assert!(1 == 1);
    }
}

/*
RSA
1.PKI 공개키
2.큰수의 소인수분해 ->양자컴퓨터 ->대부분의 암호화시스템 무용지물
3.비대칭키:누구든지 검증가능 ->블록체인
4.RSA-2048 대부분의 인터넷 뱅킹
*/

/*
큰수의 소인수분해

이산대수


*/
/*
RSA암화방식 :복호화 방식
hash방식:비밀번호 찾기 하면 무조건 바까야함


 */
/*
비트코인 암호화
1.익명성 ->ECDSA
2.부인방지 :개인키로 서명하기 떄문에  ->ECDSA
3.위변조방지 :거래 위변조를방지
*/
/*
ECC
공개키 암호기술 구현방식중 하나
RSA에 비해 더 작은 데이터로 RSA와 비슷한 보안성능
실제 디지털 서명방식으로 구현된 알고리즘을 ECDSA
비트코인에서는 secp256k1이라는 타원곡선사용


*/
/*
비트코인 address
key Conversion -> publickey ,Ucompressed Public key생성

*/
/*
Hash
mod 함수
y= x(mod n)
n= 7일떄
1= 1(mod7)
3= 10(mod 7)
6=20(mod 7)
2=30(mod 7)
5=40(mod 7)
1=50(mod 7)
일정한 임이의 256비트
단방향 알고리즘
*/
