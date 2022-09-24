use sha256::digest;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct MerkleNode {
    pub parent: Option<Rc<RefCell<MerkleNode>>>,
    pub hash: String,
}

#[derive(Debug)]
pub struct MerkleTree {
    pub leaves: Vec<MerkleNode>,
    pub root: String,
}

impl MerkleTree {
    pub fn from(input: &String) -> MerkleTree {
        let bytes = input.as_bytes();
        let mut leaves: Vec<MerkleNode> = Vec::new();
        let mut higher_level: Vec<Rc<RefCell<MerkleNode>>> = Vec::new();

        let mut i = 0;
        while i < bytes.len() - 1 {
            let hash1 = digest(bytes[i].to_string());
            let hash2 = digest(bytes[i + 1].to_string());

            let mut combined = hash1;
            combined.push_str(&hash2);

            let node_l2 = Rc::new(RefCell::new(MerkleNode {
                hash: digest(combined),
                parent: None,
            }));

            leaves.push(MerkleNode {
                hash: digest(bytes[i].to_string()),
                parent: Some(Rc::clone(&node_l2)),
            });

            leaves.push(MerkleNode {
                hash: digest(bytes[i + 1].to_string()),
                parent: Some(Rc::clone(&node_l2)),
            });

            higher_level.push(Rc::clone(&node_l2));

            i = i + 2
        }

        let root = MerkleTree::create_higher_level(higher_level);
        let root = root[0].borrow().hash.clone();

        MerkleTree { leaves, root }
    }

    fn create_higher_level(
        lower_nodes: Vec<Rc<RefCell<MerkleNode>>>,
    ) -> Vec<Rc<RefCell<MerkleNode>>> {
        let mut higher_level: Vec<Rc<RefCell<MerkleNode>>> = Vec::new();

        if lower_nodes.len() == 1 {
            higher_level.push(lower_nodes[0].clone());
            return higher_level;
        }

        let mut i = 0;
        while i < lower_nodes.len() - 1 {
            let hash1 = digest(lower_nodes[i].borrow().hash.clone());
            let hash2 = digest(lower_nodes[i + 1].borrow().hash.clone());

            let mut combined = hash1;
            combined.push_str(&hash2);

            let node_l2 = Rc::new(RefCell::new(MerkleNode {
                hash: digest(combined),
                parent: None,
            }));

            lower_nodes[i].borrow_mut().parent = Some(Rc::clone(&node_l2));
            lower_nodes[i + 1].borrow_mut().parent = Some(Rc::clone(&node_l2));

            higher_level.push(Rc::clone(&node_l2));

            i = i + 2
        }

        higher_level
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculage_merkel_tree() {
        let x = String::from("ABCD");

        let tree = MerkleTree::from(&x);
        //cargo test -- --show-output
        println!("tree is {:#?}", tree);

        //assert_eq!(x, 44);
    }
}
