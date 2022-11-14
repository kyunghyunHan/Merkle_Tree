# Merkle_Tree

- 암호학이나 컴퓨터 과학에서 머클 트리(Merkle tree) 는 모든 자식 노드들이 암호학적 해시로 이뤄진 데이터 블록을 갖는 트리 형태의 자료 구조

## SHA-256

- SHA-256은 암호화 기술로서 복호화가 되지 않는 단방향 암호화 기술

## 구조

- 트랜잭션

```rs
/*
트랜잭션
version:현재값
Flag Witnesses :Tx여부에 따라 달라짐
tx_in count :Input의 개수
tx_in :input정보
tx_out count :output의 개수
tx_out : output정보
Witnesse:  Witnesse 서명데이터
lock_time :트랜잭션 시간제한
*/
pub struct Transaction {
    version: i32,
    flag: String,
    tx_in_count: i32,
    TxIn: TxIn,
    number_of_TxOut: i32,
    TxOut: TxOut,
    witnesses: String,
    lock_time: String,
}
```

- TxIn

```rs
/*
TxIn
Transcation Hash: output이 포함된 txid
output index :Tx안에서 seq
Unlocking-script size :Unlocking-script크기
Unlocking-script: output을 input으로 바꾸는 서명정보
sequence Number :기본값 oxffffff
*/
pub struct TxIn {
    previous_output: String,
    script_bytes: String,
    signature_script: String,
    sequence: String,
}
```

- TxOut

```rs
/*
TxOut
Amount 송금할금액 사토시 단위
Locking-script size
lockking-script 송금자의 정보가 담긴 데이터
*/
pub struct TxOut {
    value: String,
    pk_script_bytes: String,
    pk_script: String,
}
```

- 머클트리

```rs
pub struct MerkleTree{
    pub nodes:
    pub levels:
}
```

## 순서

## 1.트랜잭션

## 2.트랜잭션해시

## 3.직렬화,역직렬화
