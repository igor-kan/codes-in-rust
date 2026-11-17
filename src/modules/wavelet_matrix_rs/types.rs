#[derive(Clone,
    Debug)]
pub struct Level {
    pub bits: Vec<u8>,
    pub zeros: usize,
    pub pref: Vec<usize>,
}

pub const MODULE_TAG: &str = "wavelet_matrix_rs";
