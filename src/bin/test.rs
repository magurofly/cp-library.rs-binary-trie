
use binary_trie::BinaryTrie;

fn main() {
    let mut btrie = BinaryTrie::new(4);
    for &i in &[1, 1, 2, 3, 5, 8, 13] {
      btrie.insert(i);
    }
    println!("{:?}", btrie);
}