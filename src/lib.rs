pub mod node;
use node::*;

use std::fmt::Debug;

pub struct BinaryTrie {
    xor: usize,
    bits: u32,
    /// pool[0]: root
    pool: Vec<Node>,
}
impl Debug for BinaryTrie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn dfs(pool: &[Node], f: &mut std::fmt::Formatter<'_>, depth: usize, node: usize) -> std::fmt::Result {
            let mut first_child = true;
            for dir in 0 .. 2 {
                if let Some(child) = pool[node].children[dir] {
                    if !first_child {
                        f.write_str("\n")?;
                        f.write_str(&" ".repeat(depth))?;
                    }
                    f.write_fmt(format_args!("{}", dir))?;
                    dfs(pool, f, depth + 1, child)?;
                    first_child = false;
                }
            }
            if first_child {
                f.write_fmt(format_args!(": {}", pool[node].count))?;
            }
            Ok(())
        }
        dfs(&self.pool, f, 0, 0)?;
        Ok(())
    }
}

impl std::ops::Index<usize> for BinaryTrie {
    type Output = Node;
    fn index(&self, idx: usize) -> &Node {
        &self.pool[idx]
    }
}

impl std::ops::IndexMut<usize> for BinaryTrie {
    fn index_mut(&mut self, idx: usize) -> &mut Node {
        &mut self.pool[idx]
    }
}

impl BinaryTrie {
    pub fn new(bits: u32) -> Self {
        Self {
            xor: 0,
            bits,
            pool: vec![Node::new()],
        }
    }

    fn new_node(&mut self) -> usize {
        let index = self.pool.len();
        self.pool.push(Node::new());
        index
    }

    pub fn insert(&mut self, mut x: usize) {
        x ^= self.xor;
        let mut node = 0;
        for bit in (0 .. self.bits).rev() {
            self.pool[node].count += 1;
            if self[node].children[(x >> bit) & 1].is_none() {
                self[node].children[(x >> bit) & 1] = Some(self.new_node());
            };
            node = self[node].children[(x >> bit) & 1].unwrap();
        }
        self[node].count += 1;
    }

    pub fn remove(&mut self, mut x: usize) {
        x ^= self.xor;
        let mut path = vec![];
        let mut node = 0;
        for bit in (0 .. self.bits).rev() {
            path.push(node);
            if let Some(child) = self[node].children[(x >> bit) & 1] {
                node = child;
            } else {
                return;
            }
        }
        path.push(node);
        for node in path {
            self[node].count -= 1;
        }
    }

    pub fn count_prefix(&self, mut x: usize, bits: u32) -> usize {
        x ^= self.xor;
        let mut node = 0;
        for bit in (self.bits - bits .. self.bits).rev() {
            if let Some(child) = self[node].children[(x >> bit) & 1] {
                node = child;
            } else {
                return 0;
            }
        }
        self[node].count
    }

    pub fn count(&self, x: usize) -> usize {
        self.count_prefix(x, self.bits)
    }

    pub fn contains(&self, x: usize) -> bool {
        self.count(x) > 0
    }

    pub fn xor_all(&mut self, x: usize) {
        self.xor ^= x;
    }
}


#[cfg(test)]
mod tests {
    use crate::BinaryTrie;

    #[test]
    fn it_works() {
        let mut btrie = BinaryTrie::new(4);
        btrie.insert(0b0001);
    }
}
