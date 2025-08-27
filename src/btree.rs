use std::fmt::Debug;

mod node;

#[derive(Debug)]
pub struct BTree<K: Ord + Debug + Clone, V: Debug + Clone> {
    root: Box<node::BTreeNode<K, V>>,
    node_size: usize,
}

impl<K: Ord + Debug + Clone, V: Debug + Clone> BTree<K, V> {
    pub fn new(node_size: usize) -> Self {
        BTree {
            root: Box::new(node::BTreeNode::new(true)),
            node_size,
        }
    }

    fn insert(&mut self, key: K, value: V) -> bool {
        let (ok, new_node) = self.root.insert(key, value, self.node_size);
        if ok {
            if let Some(mut new_node) = new_node {
                // 新しいrootを作り、childrenに元のrootと昇格ノードを入れる
                let keys = vec![new_node.keys.remove(0)];
                let values = vec![new_node.values.remove(0)];
                use std::mem;
                let old_root = mem::replace(&mut self.root, Box::new(node::BTreeNode::new(true)));
                let children = vec![old_root, Box::new(new_node)];
                let new_root = node::BTreeNode {
                    keys,
                    values,
                    children,
                    is_leaf: false,
                };
                self.root = Box::new(new_root);
                true
            } else {
                true
            }
        } else {
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        // arrange
        let mut btree = BTree::new(2);

        // act
        btree.insert(2, 2);
        btree.insert(1, 32);
        btree.insert(3, 1000);
        btree.insert(6, 0);
        btree.insert(4, 12);
        btree.insert(123, 78);
        btree.insert(5, 2);
        btree.insert(12, 0);
        btree.insert(111, 708);
        btree.insert(7, 10000);
        btree.insert(8, 78);
        btree.insert(13, 78);
        btree.insert(14, 78);
        btree.insert(15, 78);
        btree.insert(16, 78);
        btree.insert(20, 78);
        btree.insert(1023, 78);
        btree.insert(933, 78);
        btree.insert(2330, 78);
        println!("{:?}", btree);

        // assert
        // assert_eq!(node.keys, vec![1, 2, 5]);
        // assert_eq!(node.values, vec![2, 3, 10]);
        // assert_eq!(node.is_leaf, true);
    }
}
