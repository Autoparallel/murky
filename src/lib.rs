use tiny_keccak::Hasher;

fn keccak(data: &[u8]) -> [u8; 32] {
    let mut keccak = tiny_keccak::Keccak::v256();
    let mut hash = [0u8; 32];
    keccak.update(data);
    keccak.finalize(&mut hash);
    hash
}

#[derive(Debug)]
pub struct MerkleTree {
    leaves: Vec<String>,
    hashes: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    #[allow(clippy::new_without_default)]
    pub fn new(leaves: Vec<String>) -> Self {
        let hashes = build(leaves.clone());
        MerkleTree { leaves, hashes }
    }

    pub fn root_hash(&self) -> [u8; 32] {
        self.hashes[0][0]
    }
}

pub fn build(leaves: Vec<String>) -> Vec<Vec<[u8; 32]>> {
    let mut hashes = vec![];
    let leaf_hashes: Vec<[u8; 32]> = leaves.iter().map(|leaf| keccak(leaf.as_bytes())).collect();
    let mut branch_nodes = leaf_hashes.clone();
    hashes.push(leaf_hashes);

    // Pair up leaf hashes and hash them together to make the next level of the tree
    while branch_nodes.len() > 1 {
        let mut new_branch_nodes = vec![];
        let chunks = branch_nodes.chunks_exact(2);
        let remainder = chunks.remainder();
        for chunk in chunks {
            let combined = [chunk[0].as_slice(), chunk[1].as_slice()].concat();
            let hash = keccak(&combined);
            new_branch_nodes.push(hash);
        }
        if remainder.len() == 1 {
            let combined = [remainder[0].as_slice(), remainder[0].as_slice()].concat();
            let hash = keccak(&combined);
            new_branch_nodes.push(hash);
        }
        hashes.push(new_branch_nodes.clone());
        branch_nodes = new_branch_nodes;
    }

    hashes.reverse();
    hashes
}

impl std::fmt::Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tree_str = String::new();

        // Print the leaves
        tree_str.push_str("Leaves:\n");
        for (i, leaf) in self.leaves.iter().enumerate() {
            tree_str.push_str(&format!("  {}: {}\n", i, leaf));
        }

        // Print the hashes
        for (level, hashes) in self.hashes.iter().enumerate().rev() {
            if hashes.len() == 1 {
                tree_str.push_str("Root Hash:\n");
                tree_str.push_str(&format!("  {}\n", hex::encode(hashes[0])));
            } else {
                tree_str.push_str(&format!("Level {}:\n", level));
                for hash in hashes {
                    tree_str.push_str(&format!("  {}\n", hex::encode(hash)));
                }
            }
        }

        write!(f, "{}", tree_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn even_leaf_tree() {
        let leaves = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        println!("{}", tree);
        assert_eq!(tree.hashes.len(), 3);
        assert_eq!(tree.hashes[0].len(), 1);
        assert_eq!(tree.hashes[1].len(), 2);
        assert_eq!(tree.hashes[2].len(), 4);
    }

    #[test]
    fn odd_leaf_tree() {
        let leaves = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        println!("{}", tree);
        assert_eq!(tree.hashes.len(), 4);
        assert_eq!(tree.hashes[0].len(), 1);
        assert_eq!(tree.hashes[1].len(), 2);
        assert_eq!(tree.hashes[2].len(), 3);
        assert_eq!(tree.hashes[3].len(), 5);
    }

    #[test]
    fn test_different_value() {
        let tree1 = MerkleTree::new(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ]);
        let tree2 = MerkleTree::new(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "f".to_string(),
        ]);
        let tree3 = MerkleTree::new(vec![
            "b".to_string(),
            "a".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string(),
        ]);
        assert_ne!(tree1.root_hash(), tree2.root_hash());
        assert_ne!(tree1.root_hash(), tree3.root_hash());
        assert_ne!(tree2.root_hash(), tree3.root_hash());
    }
}
