#[derive(Clone, Debug)]
pub struct Node {
    pub key: i64,
    pub val: usize,
    pub child: Option<usize>,
    pub sibling: Option<usize>,
    pub prev: Option<usize>,
    pub active: bool,
}
