// use libc::{sysconf, _SC_PAGESIZE};
// use once_cell::sync::Lazy;
// pub static PAGE_SIZE: Lazy<i64> = Lazy::new(|| unsafe { sysconf(_SC_PAGESIZE) });

use std::fmt::Debug;

pub static NODE_SIZE: usize = 5;

pub struct BTreeNode<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
    pub children: Vec<Box<BTreeNode<K, V>>>,
    pub is_leaf: bool,
}

impl<K: Ord + Clone + Debug, V: Clone + Debug> Debug for BTreeNode<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_child(f, 0)
    }
}

impl<K: Ord + Clone + Debug, V: Clone + Debug> BTreeNode<K, V> {
    fn new(is_leaf: bool) -> Self {
        BTreeNode {
            keys: Vec::new(),
            values: Vec::new(),
            children: Vec::new(),
            is_leaf,
        }
    }

    fn new_leaf(keys: Vec<K>, values: Vec<V>) -> Self {
        BTreeNode {
            keys,
            values,
            children: Vec::new(),
            is_leaf: true,
        }
    }

    fn fmt_child(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        write!(f, "{:indent$}", "", indent = depth * 2)?;
        writeln!(f, "keys = {:?},", self.keys)?;
        write!(f, "{:indent$}", "", indent = depth * 2)?;
        writeln!(f, "values = {:?},", self.values)?;
        write!(f, "{:indent$}", "", indent = depth * 2)?;
        writeln!(f, "is_leaf = {},", self.is_leaf)?;
        if !self.children.is_empty() {
            for (i, child) in self.children.iter().enumerate() {
                write!(f, "{:indent$}", "", indent = depth * 2)?;
                writeln!(f, "children[{}]:", i)?;
                child.fmt_child(f, depth + 1)?;
            }
        }
        Ok(())
    }

    pub fn insert(&mut self, key: K, value: V) -> (bool, Option<BTreeNode<K,V>>) {
        match self.keys.binary_search(&key) {
            Ok(_idx) => {
                // すでにあれば挿入しない
                (false, None)
            }
            Err(idx) => {
                // NodeがLeafではない場合、child Nodeにいれる
                if !self.is_leaf {
                    let (ok, new_node) = self.insert_to_child_node(key, value, idx);

                    // 子ノードがされた場合、新しい子ノードを挿入
                    if let Some(new_node) = new_node {
                        // If the child was split, we need to insert the median key into this node
                        let median_key = new_node.keys[0].clone();
                        let median_value = new_node.values[0].clone();
                        self.keys.insert(idx, median_key);
                        self.values.insert(idx, median_value);
                        self.children.insert(idx + 1, Box::new(new_node));

                        // Nodeのサイズを超えないとき
                        if self.values.len() <= NODE_SIZE {
                            return (ok, None);
                        }

                        // Nodeのサイズを超えるときは分割
                        let mid_size = NODE_SIZE / 2;
                        let right_keys = self.keys.split_off(mid_size + 1);
                        let right_values = self.values.split_off(mid_size + 1);
                        let right_child = self.children.split_off(mid_size + 2);
                        let mut right_node = BTreeNode {
                            keys: right_keys,
                            values: right_values,
                            children: right_child,
                            is_leaf: self.is_leaf,
                        };
                        if right_node.children.len() == 1 {
                            let child = right_node.children[0].as_ref();
                            if right_node.keys[0] == child.keys[0] {
                                right_node.children.clear();
                                right_node.is_leaf = true;
                            }
                        }
                        return (true, Some(right_node));
                    }
                    return (ok, None);
                }

                // Leaf Nodeの場合、ここに挿入
                self.keys.insert(idx, key);
                self.values.insert(idx, value);

                if self.keys.len() <= NODE_SIZE {
                    // Nodeが収まっている場合はそのまま
                    (true, None)
                } else {
                    // Nodeが満杯の場合、分割する
                    let mid_size = NODE_SIZE / 2;
                    let right_keys = self.keys.split_off(mid_size + 1);
                    let right_values = self.values.split_off(mid_size + 1);
                    (true, Some(BTreeNode::new_leaf(right_keys, right_values)))
                }
            }
        }
    }

    fn insert_to_child_node(&mut self, key: K, value: V, child_index: usize) -> (bool, Option<BTreeNode<K, V>>) {
        if let Some(child) = self.children.get_mut(child_index) {
            let (ok, new_node) = child.insert(key, value);
            if ok {
                (true, new_node)
            } else {
                (false, None)
            }
        } else {
            self.children.push(Box::new(BTreeNode::new_leaf(vec![key], vec![value])));
            (true, None)
        }
    }

    fn insert_as_root(mut self, key: K, value: V) -> (bool, Self) {
        let (ok, new_node) = self.insert(key, value);
        if ok {
            if let Some(new_node) = new_node {
                let new_root = BTreeNode {
                    keys: vec![new_node.keys[0].clone()],
                    values: vec![new_node.values[0].clone()],
                    children: vec![Box::new(self), Box::new(new_node)],
                    is_leaf: false,
                };
                (true, new_root)
            } else {
                (true, self)
            }
        } else {
            (false, self)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        // arrange
        let node = BTreeNode::new(true);

        // act
        let (_, node) = node.insert_as_root(2, 2);
        let (_, node) = node.insert_as_root(3, 1000);
        let (_, node) = node.insert_as_root(1, 32);
        let (_, node) = node.insert_as_root(6, 0);
        let (_, node) = node.insert_as_root(4, 12);
        let (_, node) = node.insert_as_root(123, 78);
        let (_, node) = node.insert_as_root(5, 2);
        let (_, node) = node.insert_as_root(12, 0);
        let (_, node) = node.insert_as_root(111, 708);
        let (_, node) = node.insert_as_root(7, 10000);
        let (_, node) = node.insert_as_root(8, 78);
        let (_, node) = node.insert_as_root(13, 78);
        let (_, node) = node.insert_as_root(14, 78);
        let (_, node) = node.insert_as_root(15, 78);
        let (_, node) = node.insert_as_root(16, 78);
        let (_, node) = node.insert_as_root(20, 78);
        let (_, node) = node.insert_as_root(1023, 78);
        let (_, node) = node.insert_as_root(933, 78);
        let (_, node) = node.insert_as_root(2330, 78);
        println!("{:?}", node);

        // assert
        // assert_eq!(node.keys, vec![1, 2, 5]);
        // assert_eq!(node.values, vec![2, 3, 10]);
        // assert_eq!(node.is_leaf, true);
    }
}