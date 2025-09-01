// use libc::{sysconf, _SC_PAGESIZE};
// use once_cell::sync::Lazy;
// pub static PAGE_SIZE: Lazy<i64> = Lazy::new(|| unsafe { sysconf(_SC_PAGESIZE) });

use std::fmt::Debug;

#[derive(Debug)]
pub struct BTreeNode<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
    pub children: Vec<Box<BTreeNode<K, V>>>,
    pub is_leaf: bool,
}

impl<K: Ord + Clone + Debug, V: Clone + Debug> BTreeNode<K, V> {
    pub fn new(is_leaf: bool) -> Self {
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

    pub fn insert(
        &mut self,
        key: K,
        value: V,
        node_size: usize,
    ) -> (bool, Option<BTreeNode<K, V>>) {
        match self.keys.binary_search(&key) {
            Ok(_idx) => {
                // すでにあれば挿入しない
                (false, None)
            }
            Err(idx) => {
                if !self.is_leaf {
                    self.insert_as_not_leaf(key, value, idx, node_size)
                } else {
                    self.insert_as_leaf(key, value, idx, node_size)
                }
            }
        }
    }

    fn insert_as_not_leaf(
        &mut self,
        key: K,
        value: V,
        idx: usize,
        node_size: usize,
    ) -> (bool, Option<BTreeNode<K, V>>) {
        let (ok, new_node) = self.insert_to_child_node(key, value, idx, node_size);

        // 子ノードがされた場合、新しい子ノードを挿入
        if let Some(mut new_node) = new_node {
            // If the child was split, we need to insert the median key into this node
            let median_key = new_node.keys.remove(0);
            let median_value = new_node.values.remove(0);
            self.keys.insert(idx, median_key);
            self.values.insert(idx, median_value);
            if !new_node.keys.is_empty() {
                self.children.insert(idx + 1, Box::new(new_node));
            }

            if self.values.len() <= node_size {
                // Nodeのサイズを超えないとき
                (ok, None)
            } else {
                // Nodeのサイズを超えるときは分割
                let mid_size = node_size / 2;
                let right_keys = self.keys.split_off(mid_size + 1);
                let right_values = self.values.split_off(mid_size + 1);
                let right_child = if self.children.len() > mid_size + 2 {
                    self.children.split_off(mid_size + 2)
                } else {
                    Vec::new()
                };

                let right_node = BTreeNode {
                    keys: right_keys,
                    values: right_values,
                    children: right_child,
                    is_leaf: self.is_leaf,
                };

                (true, Some(right_node))
            }
        } else {
            (ok, None)
        }
    }

    fn insert_as_leaf(
        &mut self,
        key: K,
        value: V,
        idx: usize,
        node_size: usize,
    ) -> (bool, Option<BTreeNode<K, V>>) {
        // Leaf Nodeの場合、ここに挿入
        self.keys.insert(idx, key);
        self.values.insert(idx, value);

        if self.keys.len() <= node_size {
            // Nodeが収まっている場合はそのまま
            (true, None)
        } else {
            // Nodeが満杯の場合、分割する
            let mid_size = node_size / 2;
            let right_keys = self.keys.split_off(mid_size + 1);
            let right_values = self.values.split_off(mid_size + 1);
            (true, Some(BTreeNode::new_leaf(right_keys, right_values)))
        }
    }

    fn insert_to_child_node(
        &mut self,
        key: K,
        value: V,
        child_index: usize,
        node_size: usize,
    ) -> (bool, Option<BTreeNode<K, V>>) {
        if let Some(child) = self.children.get_mut(child_index) {
            let (ok, new_node) = child.insert(key, value, node_size);
            if ok {
                (true, new_node)
            } else {
                (false, None)
            }
        } else {
            self.children
                .push(Box::new(BTreeNode::new_leaf(vec![key], vec![value])));
            (true, None)
        }
    }
}
