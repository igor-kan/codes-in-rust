#[derive(Clone,
    Copy,
    Debug)]
pub struct Clause {
    pub a: usize,
    pub na: bool,
    pub b: usize,
    pub nb: bool,
}

pub const MODULE_TAG: &str = "two_sat_solver_rs";
