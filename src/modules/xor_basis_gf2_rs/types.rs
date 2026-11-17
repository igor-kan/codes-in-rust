#[derive(Clone, Debug)]
pub struct BasisState {
    pub b: [u64; 64],
    pub rank: u32,
}
impl Default for BasisState {
    fn default() -> Self {
        Self { b: [0u64; 64], rank: 0 }
    }
}

pub const MODULE_TAG: &str = "xor_basis_gf2_rs";
