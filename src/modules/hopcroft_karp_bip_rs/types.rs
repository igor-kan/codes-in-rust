#[derive(Clone, Debug)]
pub struct BipMatch {
    pub size: usize,
    pub pair_u: Vec<Option<usize>>,
}

pub const MODULE_TAG: &str = "hopcroft_karp_bip_rs";
