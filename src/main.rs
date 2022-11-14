use crate::error::BlockchainError;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
mod error;
use crypto::{digest::Digest, sha3::Sha3};
pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;
use hex;

/*
outputs
Amount 송금할금액 사토시 단위
Locking-script size
lockking-script 송금자의 정보가 담긴 데이터
*/
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Outputs {
    anount: String,
    locking_script_size: String,
    lockking_script: String,
}
/*
Inputs
Transcation Hash: output이 포함된 txid
output index :Tx안에서 seq
Unlocking-script size :Unlocking-script크기
Unlocking-script: output을 input으로 바꾸는 서명정보
sequence Number :기본값 oxffffff
*/
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Inputs {
    transaction_hash: String,
    unlocking_script_size: String,
    unlocking_script: String,
    sequence_number: String,
}
/*
트랜잭션
version:현재값
Flag Witnesses :Tx여부에 따라 달라짐
Number of inputs :Input의 개수
Inputs :input정보
Number of Outputs :output의 개수
Optputs : output정보
Witnesse:  Witnesse 서명데이터
Locktime :트랜잭션 시간제한
*/
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Transaction {
    version: i32,
    flag: String,
    number_of_inputs: i32,
    inputs: Inputs,
    number_of_outputs: i32,
    outputs: Outputs,
    witnesses: String,
    locktime: String,
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
홀수일 경우 복사해서 해시
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
//트랜잭션
impl Transaction {
    pub fn set_transaction(data: Inputs) -> Transaction {
        Transaction {
            version: 1,
            flag: "flag".to_string(),
            number_of_inputs: 1,
            inputs: data,
            number_of_outputs: 1,
            outputs: Outputs {
                anount: "amount".to_string(),
                locking_script_size: "locking_script_size".to_string(),
                lockking_script: "lockking_script".to_string(),
            },
            witnesses: "1".to_string(),
            locktime: "1".to_string(),
        }
    }
}
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
    // assert!(true);
    // //트랜잭션데이터
    // let tx1 = Transaction {
    //     id: "1".to_string(),
    //     vin: "2".to_string(),
    //     vout: "3".to_string(),
    // };
    // let tx2 = Transaction {
    //     id: "4".to_string(),
    //     vin: "5".to_string(),
    //     vout: "6".to_string(),
    // };
    // //트랜잭션 해시 및 직렬화
    // let hash_tx1 = MerkleTree::transaction_hash(&tx1);
    // let hash_tx2 = MerkleTree::transaction_hash(&tx2);
    // println!("트랜잭션 해시 및 직렬화:{:?}", hash_tx1);

    // //해시 집합 및 직렬화
    // let mut txs = Vec::new();
    // txs.push(bincode::serialize(&hash_tx1).unwrap());
    // txs.push(bincode::serialize(&hash_tx2).unwrap());
    // txs.push(bincode::serialize(&hash_tx1).unwrap());
    // txs.push(bincode::serialize(&hash_tx2).unwrap());

    // println!("해시 집합:{:?}", txs);

    // //연결
    // let concat = MerkleTree::hash_concat(&txs[0], &txs[1]);
    // println!("연결{:?}", concat);
    // // // let tss = MerkleTree::construct_level_up(&txs);

    // //연결한 해시들 해시
    // let hash = hash_to_str(&concat);
    // println!("연결한 해시들 해시:{:?}", hash);
    // //해시를 다시 직렬
    // let ser_hash1 = bincode::serialize(&hash).unwrap();
    // println!("해시를 다시 직렬 {:?}", ser_hash1);
    // let ser_hash2 = bincode::serialize(&hash).unwrap();
    // println!("해시를 다시 직렬 {:?}", ser_hash2);

    // //역직렬화
    // let deserialize_hash: String = bincode::deserialize(&ser_hash1).unwrap();
    // println!("{:?}", deserialize_hash);

    // let mekle_root = MerkleTree::mekle_tree_return(txs).hash;
    // println!("merkle_root:{:?}", mekle_root);

    // let test = [0];
    // let test_hash1 = bincode::serialize(&test).unwrap();
    // let test2 = 0;
    // let test_hash2 = bincode::serialize(&test2).unwrap();

    // println!("1{:?}", test_hash1);
    // println!("2{:?}", test_hash2);

    // let ara = MerkleTree::construct_level_up(&txs);

    let input = "090A0B0C";
    let add = serialize(input);
    let decoded = hex::decode(input).expect("Decoding failed");
    println!("{:?}", decoded);
    let encoded = hex::encode(decoded);
    println!("{:?}", encoded);
    let data = Inputs {
        transaction_hash: "1".to_string(),
        unlocking_script_size: "1".to_string(),
        unlocking_script: "1".to_string(),
        sequence_number: "1".to_string(),
    };
    let test = Transaction::set_transaction(data);
    println!("{:?}", test);
}
//TDD
#[cfg(test)]
mod tests {
    use super::*;
    /*트랜잭션 해시되는지 */
    #[test]
    fn test1() {
        let tx1 = Transaction {
            version: 1,
            flag: "s".to_string(),
            number_of_inputs: 1,
            inputs: Inputs {
                transaction_hash: "s".to_string(),
                unlocking_script_size: "s".to_string(),
                unlocking_script: "s".to_string(),
                sequence_number: "s".to_string(),
            },
            number_of_outputs: 1,
            outputs: Outputs {
                anount: "s".to_string(),
                locking_script_size: "s".to_string(),
                lockking_script: "s".to_string(),
            },
            witnesses: "s".to_string(),
            locktime: "s".to_string(),
        };
        let tx2 = Transaction {
            version: 1,
            flag: "s".to_string(),
            number_of_inputs: 1,
            inputs: Inputs {
                transaction_hash: "s".to_string(),
                unlocking_script_size: "s".to_string(),
                unlocking_script: "s".to_string(),
                sequence_number: "s".to_string(),
            },
            number_of_outputs: 1,
            outputs: Outputs {
                anount: "s".to_string(),
                locking_script_size: "s".to_string(),
                lockking_script: "s".to_string(),
            },
            witnesses: "s".to_string(),
            locktime: "s".to_string(),
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

/*
newwork

client server
client -> Gateway  -> was -> server -> database



p2p Network
- 어느서버에다가도 하던지 동일한 데이터를 받을수 있다
- 토렌트


*/
/*
TCP
서버와 client간에 데이터를 신뢰성 있게 전달하기 위해 만들어진 프로토콜

데이터 전송을 위한 연결을 만드는 연결지향 프로토콜

데이터 전송 과정에서 손실이나 순서가 바뀌는 경우 교정 및 순사 재조합 지원

IPC소켓통신 방법으로 보통 지원


*/
/*
http
get
post
put

delete
head  서버 헤드 정보 획득 요청
options 서버 옵션 확인 요창

*/
/*
web socket
양방향
한별연결이 수립되면 클라이언트와 서버 자유롭게 데이터 전송가능
실시간 시세 데이터 ,채팅 솔루션 등에 사용
*/
/*
RPC
원격 서버의 함수를 함출 호출할수 있는 통신기술
IDL을 사용해서 호출 규약을 정의하고 이를 통해 stub코드를 생성
program 에서는 stud을 call함으로써 개발자는 네트워크의 대한 지식 업이 원격 함수 호출이 가능하다.
*/
/*
gRPC
구글에서 개발한 RPC통신
양방향 스트링 데이터 처리 MSA구조의 서비스에서 활용

protobuf
grpc의 IDL protobuffer의 줄임말 프로그램 상에서 이를 사용하기 위해 .proto stub이 생성되어야 한다.json,xml통신보다 데이터 전송 크기가 작고 성능이 빠르다.

proto3를 사용
*/
/*
블록 검증

신규블록 수신
블록구조 일치여부
재계산 block header hash== block header hash
block timestamp <now()+2hours
block size <1mb
coinbace transaction check
transaction check
mempool update  ->데이터베이스 업데이트
levelDB insert New Block
Block 전파



*/
/*
트랙잭션 전피

트랜잭션을 다른 노드에게서 전파받는다
이미 받은 트랜잭션인지 확인
)없는 경우 다른 노드에게 전파
상대노드가 없는 경우 getdata를 요청받는다
새로운 트랜잭션에 전달
연결딘 모든 노드에게 전달될떄까지 수행한다

*/
/*

트랜잭션 검증

신규 투랜잭션 수신
트랜잭션 구조 일치 여부
in,out list 존재여부

트랜잭션 사이즈 <1mb
output value <2100만 btc
mempool존재 여부
block 존재여부
input check(double spending)
input check(orphan tx)
input check(coin base) 보상
input check(Not UTXO)
input >output value
check input script
add mempool
트랜잭션 전파
*/
/*
블록전파
- 마이닝에 성공한 블록은 아래 방법 없이 블록 전체 데이터를 모든 노드에게 전달
- network에 블록체인 다운 받기 위해 언결된 다른 노드들에 ping전송
- 전달받은 block header전달
- 아직 전달 받지 못한 block인 경우 header와 getdata를 모두 요처
새로운 블록전달
*/

/*

블록 구조

 Block size   4bytes    <1mb
BlockHeader 80bytes
Transaction Counter 1~9 bytes
Transaction variable

*/
/*
블록헤더 구조
version 버전정보
previous block hash 이전 블록의 헤더 해시
merkle root 트랜잭션들의 hashroot
timestamp 블록 생성시간
Difficulty Target pow의 어려움 정도
nonce
*/
/*
블록생성

mempool tx선택
coinbase tx 생성
merkle root 연산
block header구성
find nonce
block전파
*/
/*
UTXO
아직 사용되지 않는 Output을 지칭
UTXO 사용 여부를 통해서 자산의 안정성을 확인
input 사용자가 내는금액
output 받는금액
*/
/*
트랜잭션 내부구조

version  현재값1
Flag  Witnesses Tx여부에 따라 달라짐
Number of inputs input의 개수
inputs input정보
Number of Outputs  ouput의 개수
outputs output정보
Witnesses Witnesses서명데이터
Locktime 트랜잭션 시간 제한


*/
/*
input구조
Transcation 해시 output이 포함된 txid
output index  Tx안에서 seq
Unlocking-script size Unlocking-script크기
Unlocking-script  output을 input으로 바꾸는 서명정보
sequence Number  기본값 oxffffff

*/
/*
Amount  송금할금액 사토시 단위
Locking-script size
lockking-script   송금자의 정보가 담긴 데이터

*/
/*
Transaction Fee
input 총힙에서 전체 output의 총합을 뺸 값
블록에서 설명했듯니 채굴자들이 거래를 더 빠르게 하기 위해서 수수료를 높여야 한다

*/

/*
coinbase

pow에서 채굴에 성공하게 되면 채굴에 성공한 채굴자에게 기본 보상 수수료와 거래 수수료를 보상으로 제공
//거래방식
*/
/*
p2pk
우웃풋
이전 아웃풋에 포함된 공개키
op_checksig

input
서명
*/
/*
p2pkh
퍼블릿키 해시값

*/
/*
NULL_DATA
블록체인상에 데이터를 저장하는 방식
input scriptsig가 들어가지 않는 방식
OP_RETURN을 사용

*/
/*
BITCOINT 새로운거래형식
SEGWIT
p2pkh랑 비슷
*/
/*
TapRoot
2021년부터 업그레이드로 인해 새로운 거래형식
슈노르 서명방식 지원
- 공동 공개키를 셍성하여 하나의 서명으로 공동서명
MAST지원
비트코인 스크립트 실행사실을 숨길수 있음
비트코인 프라이버시를 향상시키고 트랜잭션의 수수료를 감소

*/
/*
Lightning
NEWwork



비트코인 레이어 2기술로 블록체인 상에서 일정 금액을 생성하고
이를 네트워크 상에 배포시키지 않고 잠금된 금액 기반으로 실시간 거래가 가능하도록 하는 기술

엘살바도르 국민들은 이 기술을 비트콘인 법정 화폐

*/

/*
비잔틴 장군문제
특정수 이상의 장군이 동시에 공격을 해야 성을 공략할수 있다.
서로 p2p만 연락을 주고 받을떄 첩자의 방해가 있더라도 이 공격을 성공시키는 방법은?


proof of work

BFT
*/
/*
BFT
분산화된 네트워크에서 일부장애가
발생하더라도
네트워크가 정상적으로 동작하도록하는 알고리즘
PBFT가 블록체인생태계에서 사용
Cosmos,하이페브릭
*/

/*
proof of work
컴퓨팅 파워로 doble spending과 같은 거래 위변조 공격을 막는방법

새로운 블록을 생성하는것이고 그 블록내에 field로 포함되는 nonce값을 찾는것
전체 Network hash에 따라 Difficulty가 변화하고 10분마다 block이 생성되게 조정
*/

/*
채굴과정
새로운 블록이 생성됨을 알림받는다
다음 블록 생성을 위해서 임시pending중인 트랜잭셕을 포함한다
Coinbase거래를 임시 블록에 포함한다
이번 블록 a와 트렌잭션들을 포함한 임시 블록 b를 만든다


*/

/*
Network Hash Rate와 Difficulty
Miner참여자 수가 증가하고 성능이 좋은 채굴 장비를 이용하게 되면 채굴의 속도가 점점 빨라진다.
Difficulty따라 Bit가 조절되고 정답이 되는 Header Hash 의 0의 개수가 늘어난다
*/

/*
Find Nonce
bits ->hex값으로변경

0x29D72D *22 **(8*(0x17 -3))
= 0x00000
header hash가 targe
*/
/*
채굴보상
전체 2100만개

*/
/*
비트코인 공격방식
51%어택

동일한 utxo로 두개의 거래를 생성하고 fork를 통해서 공격자가 원하는 거래만 블록에 포함되게 하는 공격
공격자가 더 긴 블록체인을 만들기 위해서는 전체 네트워크 hashRate의 51프로를 가져야 성공 가능성이 높음


*/
/*
Sybul attack/ dos attic

- 공격자가 수많은 노드를 운영하면서 비트코인 네트워크 block전파를 방해하거나 잘못된 block data를 인접 노드들에게 전송하는 공격
- 특정 노드들에게 비정상적인 거래를 무한정 생성되어 네트워크 전체의 마비를 이르키는 공격

- 비정상적인 거래 블록은 전파하지 않음
- 이중 지불 공격은 전파하지 않음
- 같은 노드에서 전송된 동일블록과 거레는 전파하지 않음
- 아주 작은 단위의 거래를 전송


*/
/*
Longest chain rule

블록체인 네트워크 전체가 fork가 발생할 떄 하나의 블록체인만을 유지하기 위한 방법


*/

/*
Asic과
특정용도에 맞게 맞춤 제작된 집적 회로를 의미


Mining Pool
고성능 장비를 구매하기 위한 일반 사용자들이 모여서 채굴에 참여하기 위해 등장


share - 지분투입정보
pay-per-share - 보상에 지분에 따라 지급하는 방식
solo miningpool - 찾은사람이 다갖는
채굴시 일정 지분 등록하고 연산한만큼

*/
/*
Level db
kb database
관계형 검색이 불가능
하나의 프로세스만이 특정 데이터 베이스 접근가능
읽기 쓰기 성능이 빠르다


입력
조회
삭제



'b'+32-byte block hash      /Block index기록
'f'+4-byte file number      /파일 정보기록
'i'+4-byte file number
'R'+1-byte boolean      /Reindexing여부
'F'+1-byte flag nama length+flag name string  /Txindex On/Off여부
't'+32-byte transaction hash       /Transaction index기록

'c' +32 byte transaction hash  /트랜잭션 내 UTXO 데이터 조회용
'B' -> 32 -byte block hash  /가장 최신 Block이 있는지확인용
*/

/*
Mempool

아직 블록에 포함되지 않는 pending Transaction들을 저장 및 관리하는 방법
채굴자들은 Mempool중에서 Transaction을 선택해서 신규 Block에 포함시킨다
Mempool에 들어가고도 14일동안 처리되지 않고 남아 있는 Transaction은 Expired된다

*/
/*
key값으로만 검색이 가능

*/
/*
Fork

동시에 블록정답을 찾기에 성공하게 된경우를 분기되었다 또는 Fork라 부른다
Longest Blockchain Rule을 통해 Fork된 네트워크 를 하나로 유지시키고 있다.






*/
