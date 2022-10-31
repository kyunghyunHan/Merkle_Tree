use sha2::Digest;

//data값
pub type Data = Vec<u8>;
//hash
pub type Hash = Vec<u8>;

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

#[derive(Debug)]
pub enum Error {
    CantFindDataInMerkleTree,
    IndexIsNotALeaf,
}

type Result<T> = std::result::Result<T, Error>;
//머클트리
impl MerkleTree {
    fn construct_level_up(level: &[Hash]) {

        // 하위 해시를 연결하여 상위 레벨을 찾고 이전 레벨로 이동합니다.
    }

    /// 주어진 입력 데이터로부터 머클 트리 생성
    pub fn construct(input: &[Data]) {
        // 일반적으로 주장하는 대신 여기에 결과를 반환하지만
        // 제공된 함수 서명으로 유지

        // 입력 데이터의 해시를 가져온다. 이것은 Merkle 트리의 잎이 된다.

        // 한 번에 한 레벨씩 트리를 반복하고 다음 레벨에서 노드를 계산한다.
    }

    /// 주어진 입력 데이터가 주어진 루트 해시를 생성하는지 확인
    pub fn verify(input: &[Data], root_hash: &Hash) {}

    /// 머클 트리의 루트 해시를 반환합니다.
    pub fn root_hash(&self) {}

    /// Merkle 트리를 구성하는 데 사용된 데이터의 수를 반환
    pub fn num_leaves(&self) {}

    /// 머클 트리의 levels(기본 데이터의 해시)를 반환
    fn leaves(&self) {}

    /// 주어진 노드 인덱스의 부모 노드 인덱스를 반환
    fn parent_index(&self, index: usize) -> usize {
        // 이 함수는 내부적으로만 사용해야 하므로 여기에서 주장하는 것이 좋다.

        index
    }
    /// 주어진 리프 인덱스에 대한 머클 증명을 생성합니다.
    /// 인덱스가 리프에 해당하지 않으면 오류를 반환합니다.
    pub fn get_merkle_proof_by_index(&self, leaf_index: usize) {

        // 이미 한 쪽의 해시를 알고 있거나 이미 계산할 수 있다.
        // 쌍이므로 증명을 위해 다른 하나를 반환해야 한다.

        // 이제 부모의 해시를 계산할 수 있으므로 부모의
        // 이 노드는 이제 알려진 노드
    }

    /// 주어진 데이터의 첫 번째 발생에 대한 Merkle 증명을 생성
    /// 머클 트리에서 데이터를 찾을 수 없으면 오류를 반환.
    pub fn get_merkle_proof_by_data(&self, data: &Data) {}
}

/// 주어진 증명이 주어진 루트 해시와 데이터에 유효한지 확인
pub fn verify_merkle_proof(proof: &Proof, data: &Data, root_hash: &Hash) {}

fn hash_data(data: &Data) -> Hash {
    sha2::Sha256::digest(data).to_vec()
}

//연결
// fn hash_concat(h1: &Hash, h2: &Hash) -> Hash {
//     let h3 = h1.iter().chain(h2).copied().collect();
//     hash_data(&h3)
// }

fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}
fn main() {
    let data = vec![
        Data::from("AAAA"),
        Data::from("BBBB"),
        Data::from("CCCC"),
        Data::from("DDDD"),
        Data::from("EEEE"),
        Data::from("FFFF"),
        Data::from("GGGG"),
        Data::from("HHHH"),
        Data::from("abcd"),
    ];
    println!("{:?}", hash_data(&data[0]))
}

//Test
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn testttt() {
//         assert_eq!((is_power_of_two(2)), true)
//     }
//     #[test]
//     fn testttt2() {
//         let data = vec![
//             Data::from("AAA"),
//             Data::from("BBB"),
//             Data::from("CCC"),
//             Data::from("DDD"),
//             Data::from("AAA"),
//         ];
//         assert_eq!((hash_data(&data[0])), hash_data(&data[4]))
//     }
//     #[test]
//     fn testttt3() {
//         let data = vec![
//             Data::from("AAA"),
//             Data::from("BBB"),
//             Data::from("CCC"),
//             Data::from("DDD"),
//             Data::from("AAA"),
//         ];
//         assert_eq!((hash_data(&data[0])), hash_data(&data[4]))
//     }
// }
