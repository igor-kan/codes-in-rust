#[derive(Debug, Default, Clone)]
pub struct XorTrieNode {
    pub child: [usize; 2],
}

pub const MODULE_TAG: &str = "trie_xor_max_pair_d56";
