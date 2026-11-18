#[derive(Clone, Debug)]
pub struct EertreeNode {
    pub len: isize,
    pub link: usize,
    pub next: [usize; 26],
}

pub const MODULE_TAG: &str = "eertree_palindromic_rs";
