use std::prelude::v1::*;
use crypto::sha3::Sha3;

use tree::Tree;
use hashutils::HashUtils;

/// An inclusion proof represent the fact that a `value` is a member
/// of a `MerkleTree` with root hash `root_hash`, and hash function `algorithm`.
#[derive(Clone)]
pub struct Proof<T> {

    /// The hashing algorithm used in the original `MerkleTree`
    pub algorithm: Sha3,

    /// The hash of the root of the original `MerkleTree`
    pub root_hash: Vec<u8>,

    /// The first `Lemma` of the `Proof`
    pub lemma: Lemma,

    /// The value concerned by this `Proof`
    pub value: T
}

impl <T> Proof<T> {

    /// Constructs a new `Proof`
    pub fn new(algo: Sha3, root_hash: Vec<u8>, lemma: Lemma, value: T) -> Self {
        Proof {
            algorithm: algo,
            root_hash: root_hash,
            lemma: lemma,
            value: value
        }
    }

    /// Checks whether this inclusion proof is well-formed,
    /// and whether its root hash matches the given `root_hash`.
    pub fn validate<H>(&self, root_hash: &[u8]) -> bool
        where Sha3: HashUtils<H>, H: AsRef<[u8]> {
        if self.root_hash != root_hash || self.lemma.node_hash != root_hash {
            return false
        }

        self.validate_lemma(&self.lemma)
    }

    fn validate_lemma<H>(&self, lemma: &Lemma) -> bool
        where Sha3: HashUtils<H>, H: AsRef<[u8]> {
        match lemma.sub_lemma {

            None =>
                lemma.sibling_hash.is_none(),

            Some(ref sub) =>
                match lemma.sibling_hash {
                    None =>
                        false,

                    Some(Positioned::Left(ref hash)) => {
                        let combined = self.algorithm.hash_nodes(hash, &sub.node_hash);
                        let hashes_match = combined.as_ref() == lemma.node_hash.as_slice();
                        hashes_match && self.validate_lemma(sub)
                    }

                    Some(Positioned::Right(ref hash)) => {
                        let combined = self.algorithm.hash_nodes(&sub.node_hash, hash);
                        let hashes_match = combined.as_ref() == lemma.node_hash.as_slice();
                        hashes_match && self.validate_lemma(sub)
                    }

                }
        }
    }

}


/// A `Lemma` holds the hash of a node, the hash of its sibling node,
/// and a sub lemma, whose `node_hash`, when combined with this `sibling_hash`
/// must be equal to this `node_hash`.
#[derive(Clone, Debug, PartialEq)]
pub struct Lemma {
    pub node_hash: Vec<u8>,
    pub sibling_hash: Option<Positioned<Vec<u8>>>,
    pub sub_lemma: Option<Box<Lemma>>
}

impl Lemma {

    /// Attempts to generate a proof that the a value with hash `needle` is a member of the given `tree`.
    pub fn new<T>(tree: &Tree<T>, needle: &[u8]) -> Option<Lemma> {
        match *tree {
            Tree::Empty {.. } =>
                None,

            Tree::Leaf { ref hash, .. } =>
                Lemma::new_leaf_proof(hash, needle),

            Tree::Node { ref hash, ref left, ref right } =>
                Lemma::new_tree_proof(hash, needle, left, right)
        }
    }

    fn new_leaf_proof(hash: &[u8], needle: &[u8]) -> Option<Lemma> {
        if *hash == *needle {
            Some(Lemma {
                node_hash: hash.into(),
                sibling_hash: None,
                sub_lemma: None
            })
        } else {
            None
        }
    }

    fn new_tree_proof<T>(hash: &[u8], needle: &[u8], left: &Tree<T>, right: &Tree<T>) -> Option<Lemma> {
        Lemma::new(left, needle)
            .map(|lemma| {
                let right_hash = right.hash().clone();
                let sub_lemma = Some(Positioned::Right(right_hash));
                (lemma, sub_lemma)
            })
            .or_else(|| {
                let sub_lemma = Lemma::new(right, needle);
                sub_lemma.map(|lemma| {
                    let left_hash = left.hash().clone();
                    let sub_lemma = Some(Positioned::Left(left_hash));
                    (lemma, sub_lemma)
                })
            })
            .map(|(sub_lemma, sibling_hash)| {
                Lemma {
                    node_hash: hash.into(),
                    sibling_hash: sibling_hash,
                    sub_lemma: Some(Box::new(sub_lemma))
                }
            })
    }

}

/// Tags a value so that we know from which branch of a `Tree` (if any) it was found.
#[derive(Clone, Debug, PartialEq)]
pub enum Positioned<T> {

    /// The value was found in the left branch
    Left(T),

    /// The value was found in the right branch
    Right(T)
}
