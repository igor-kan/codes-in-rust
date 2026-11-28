#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EditStats {
    pub distance: usize,
    pub pattern_len: usize,
}

pub const MODULE_TAG: &str = "myers_bit_edit_rs_d61";
