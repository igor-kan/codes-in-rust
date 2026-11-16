#[derive(Debug, Clone)]
pub struct VebTree {
    pub u: usize,
    pub shift: u32,
    pub mask: usize,
    pub min: Option<usize>,
    pub max: Option<usize>,
    pub summary: Option<Box<VebTree>>,
    pub clusters: Vec<VebTree>,
}
