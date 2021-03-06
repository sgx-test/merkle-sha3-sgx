
#![cfg(test)]
use std::prelude::v1::*;
use crypto::sha3::Sha3;
use crypto::digest::Digest;

use merkletree::MerkleTree;
use hashutils::{ Hashable, HashUtils };
use proof::Positioned;

#[test]
fn test_from_str_vec() {
    let values = vec!["one", "two", "three", "four"];
    let digest = Sha3::keccak256();

    let hashes: Vec<[u8; 32]> = vec![
        digest.hash_leaf(&values[0].as_bytes()),
        digest.hash_leaf(&values[1].as_bytes()),
        digest.hash_leaf(&values[2].as_bytes()),
        digest.hash_leaf(&values[3].as_bytes())
    ];

    let count = values.len();
    let tree = MerkleTree::from_vec::<[u8; 32]>(digest, values);

    let h01: [u8; 32] = digest.hash_nodes(&hashes[0], &hashes[1]);
    let h23: [u8; 32] = digest.hash_nodes(&hashes[2], &hashes[3]);

    let root_hash: [u8; 32] = digest.hash_nodes(&h01, &h23);

    assert_eq!(tree.count(), count);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}


#[test]
fn test_from_vec_empty() {
    let values: Vec<Vec<u8>> = vec![];
    let digest = Sha3::keccak256();
    let tree = MerkleTree::from_vec::<[u8; 32]>(digest, values);
    let empty_hash_a: [u8; 32] = digest.hash_empty();
    let empty_hash: Vec<u8> = empty_hash_a.as_ref().into();
    let root_hash= tree.root_hash().clone();

    assert_eq!(root_hash, empty_hash);
}

#[test]
fn test_from_vec1() {
    let values = vec!["hello, world".to_string()];
    let digest = Sha3::keccak256();
    let tree   = MerkleTree::from_vec::<[u8; 32]>(digest, values);

    let root_hash: [u8; 32] = digest.hash_leaf(&"hello, world".as_bytes());

    assert_eq!(tree.count(), 1);
    assert_eq!(tree.height(), 0);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}


#[test]
fn test_from_vec3() {
    let values = vec![vec![1], vec![2], vec![3]];
    let digest = Sha3::keccak256();
    let tree   = MerkleTree::from_vec::<[u8; 32]>(digest, values);

    let hashes: Vec<[u8; 32]> = vec![
        digest.hash_leaf(&vec![1]),
        digest.hash_leaf(&vec![2]),
        digest.hash_leaf(&vec![3])
    ];

    let h01: [u8; 32] = digest.hash_nodes(&hashes[0], &hashes[1]);
    let h2 = &hashes[2];
    let root_hash: [u8; 32] = digest.hash_nodes(&h01, h2);

    assert_eq!(tree.count(), 3);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}

#[test]
fn test_from_vec9() {
    let values = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let digest = Sha3::keccak256();
    let tree   = MerkleTree::from_vec::<[u8; 32]>(digest, values.clone());

    let hashes = values.iter().map(|v| digest.hash_leaf(v)).collect::<Vec<_>>();

    let h01: [u8; 32] = digest.hash_nodes(&hashes[0], &hashes[1]);
    let h23: [u8; 32] = digest.hash_nodes(&hashes[2], &hashes[3]);
    let h45: [u8; 32] = digest.hash_nodes(&hashes[4], &hashes[5]);
    let h67: [u8; 32] = digest.hash_nodes(&hashes[6], &hashes[7]);
    let h8 = &hashes[8];
    let h0123: [u8; 32] = digest.hash_nodes(&h01, &h23);
    let h4567: [u8; 32] = digest.hash_nodes(&h45, &h67);
    let h1to7: [u8; 32] = digest.hash_nodes(&h0123, &h4567);

    let root_hash: [u8; 32] = digest.hash_nodes(&h1to7, h8);

    assert_eq!(tree.count(), 9);
    assert_eq!(tree.height(), 4);
    assert_eq!(tree.root_hash().as_slice(), root_hash.as_ref());
}

#[test]
fn test_valid_proof() {
    let values    = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let digest    = Sha3::keccak256();
    let tree      = MerkleTree::from_vec::<[u8; 32]>(digest, values.clone());
    let root_hash = tree.root_hash();

    for value in values {
        let proof    = tree.gen_proof::<[u8; 32]>(value);
        let is_valid = proof.map(|p| p.validate::<[u8; 32]>(root_hash)).unwrap_or(false);

        assert!(is_valid);
    }
}

#[test]
fn test_valid_proof_str() {
    let values    = vec!["Hello", "my", "name", "is", "Rusty"];
    let digest    = Sha3::keccak256();
    let tree      = MerkleTree::from_vec::<[u8; 32]>(digest, values);
    let root_hash = tree.root_hash();

    let value = "Rusty";

    let proof    = tree.gen_proof::<[u8; 32]>(&value);
    let is_valid = proof.map(|p| p.validate::<[u8; 32]>(&root_hash)).unwrap_or(false);

    assert!(is_valid);
}

#[test]
fn test_wrong_proof() {
    let digest    = Sha3::keccak256();

    let values1   = vec![vec![1], vec![2], vec![3], vec![4]];
    let tree1     = MerkleTree::from_vec::<[u8; 32]>(digest, values1.clone());

    let values2   = vec![vec![4], vec![5], vec![6], vec![7]];
    let tree2     = MerkleTree::from_vec::<[u8; 32]>(digest, values2.clone());

    let root_hash = tree2.root_hash();

    for value in values1 {
        let proof    = tree1.gen_proof::<[u8; 32]>(value);
        let is_valid = proof.map(|p| p.validate::<[u8; 32]>(root_hash)).unwrap_or(false);

        assert_eq!(is_valid, false);
    }
}

#[test]
fn test_mutate_proof_first_lemma() {
    let values    = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let digest    = Sha3::keccak256();
    let tree      = MerkleTree::from_vec::<[u8; 32]>(digest, values.clone());
    let root_hash = tree.root_hash();

    let mut i = 0;

    for value in values {
        let mut proof = tree.gen_proof::<[u8; 32]>(value).unwrap();

        match i % 3 {
            0 => {
                proof.lemma.node_hash = vec![1, 2, 3];
            },
            1 => {
                proof.lemma.sibling_hash = Some(Positioned::Left(vec![1, 2, 3]));
            },
            _ => {
                proof.lemma.sibling_hash = Some(Positioned::Right(vec![1, 2, 3]));
            }
        }

        let is_valid = proof.validate::<[u8; 32]>(root_hash);
        assert_eq!(is_valid, false);

        i += 1;
    }
}

#[test]
fn test_tree_iter() {
    let values  = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let digest  = Sha3::keccak256();
    let tree    = MerkleTree::from_vec::<[u8; 32]>(digest, values.clone());
    let iter    = tree.iter().map(|x| x.clone()).collect::<Vec<_>>();

    assert_eq!(values, iter);
}

#[test]
fn test_tree_into_iter() {
    let values  = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let digest  = Sha3::keccak256();
    let tree    = MerkleTree::from_vec::<[u8; 32]>(digest, values.clone());
    let iter    = tree.into_iter().map(|x| x.clone()).collect::<Vec<_>>();

    assert_eq!(values, iter);
}

#[test]
fn test_tree_into_iter_loop() {
    let values  = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let digest  = Sha3::keccak256();
    let tree    = MerkleTree::from_vec::<[u8; 32]>(digest, values.clone());

    let mut collected = Vec::new();

    for value in tree {
        collected.push(value);
    }

    assert_eq!(values, collected);
}

#[test]
fn test_tree_into_iter_loop_borrow() {
    let values  = (1..10).map(|x| vec![x]).collect::<Vec<_>>();
    let digest  = Sha3::keccak256();
    let tree    = MerkleTree::from_vec::<[u8; 32]>(digest, values.clone());

    let mut collected = Vec::new();

    for value in &tree {
        collected.push(value);
    }

    let refs = values.iter().collect::<Vec<_>>();

    assert_eq!(refs, collected);
}


pub struct PublicKey {
    zero_values: Vec<Vec<u8>>,
    one_values: Vec<Vec<u8>>
}

impl PublicKey {

    pub fn new(zero_values: Vec<Vec<u8>>, one_values: Vec<Vec<u8>>) -> Self {
        PublicKey {
            zero_values: zero_values,
            one_values: one_values
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.zero_values.iter().chain(self.one_values.iter())
            .fold(Vec::new(), |mut acc, i| {
                acc.append(&mut i.clone());
                acc
            })
    }
}

impl Hashable for PublicKey {

    fn update_context(&self, context: &mut Sha3) {
        context.input(&self.to_bytes());
    }

}

#[test]
fn test_custom_hashable_impl() {
    let keys = (0..10).map(|i| {
        let zero_values = vec![vec![i], vec![i + 1], vec![i + 2]];
        let one_values  = vec![vec![i + 3], vec![i + 4], vec![i + 5]];

        PublicKey::new(zero_values, one_values)
    }).collect::<Vec<_>>();

    let digest = Sha3::keccak256();
    let tree = MerkleTree::from_vec::<[u8; 32]>(digest, keys);

    assert_eq!(tree.count(), 10);
    assert_eq!(tree.height(), 4);
}
