#[derive(Debug, Clone)]
pub struct MaglevRouter {
    pub table_size: usize,
    pub backends: Vec<String>,
    pub lookup: Vec<usize>,
}
