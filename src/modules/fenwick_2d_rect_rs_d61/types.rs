#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RectQuery {
    pub x1: usize,
    pub y1: usize,
    pub x2: usize,
    pub y2: usize,
}

pub const MODULE_TAG: &str = "fenwick_2d_rect_rs_d61";
