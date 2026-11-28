#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrimeReport {
    pub candidate: u64,
    pub prime: bool,
}

pub const MODULE_TAG: &str = "miller_rabin_u64_rs_d61";
