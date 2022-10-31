// use crypto::{digest::Digest, sha3::Sha3};
use sha2::Digest;
pub type Data = Vec<u8>;
pub type Hash = Vec<u8>;

//nodes:각 레이어에 존재하는 노드들
//levels:자식 노드가 없는 최하단 노드들
pub struct MerkleTree {
    pub nodes: Vec<Hash>,
    pub levels: usize,
}
//증명 해시를 연결할 때 해시를 넣을 쪽에
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HashDirection {
    Left,
    Right,
}

#[derive(Debug, Default)]
pub struct Proof<'a> {
    /// 증명을 확인할 때 사용할 해시
    /// 튜플의 첫 번째 요소는 연결할 때 해시가 있어야 하는 쪽입니다.
    hashes: Vec<(HashDirection, &'a Hash)>,
}
#[derive(Debug)]
pub enum Error {
    CantFindDataInMerkleTree,
    IndexIsNotALeaf,
}
type Result<T> = std::result::Result<T, Error>;

impl MerkleTree {
    //하위 해시를 연결하여 상위 레벨을 찾고 이전 레벨로 이동

    fn construct_level_up(level: &[Hash]) -> Vec<Hash> {
        // assert!(is_power_of_two(level.len()));

        //하위 해시를 연결하여 상위 레벨을 찾고 이전 레벨로 이동
        level
            .chunks(2)
            .map(|pair| hash_concat(&pair[0], &pair[1]))
            .collect()
    }
}
fn hash_concat(h1: &Hash, h2: &Hash) -> Hash {
    let h3 = h1.iter().chain(h2).copied().collect();
    hash_data(&h3)
}
fn hash_data(data: &Data) -> Hash {
    let result = sha2::Sha256::digest(data).to_vec();
    println!("{:?}", &result);
    result
}
fn is_power_of_two(n: usize) -> bool {
    n != 0 && (n & (n - 1)) == 0
}

fn main() {}

//Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testttt() {
        assert_eq!((is_power_of_two(2)), true)
    }
    #[test]
    fn testttt2() {
        let data = vec![
            Data::from("AAA"),
            Data::from("BBB"),
            Data::from("CCC"),
            Data::from("DDD"),
            Data::from("AAA"),
        ];
        assert_eq!((hash_data(&data[0])), hash_data(&data[4]))
    }
}
