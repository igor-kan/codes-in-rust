pub trait VerifyTag {
    fn tag() -> &'static str;
}

pub struct DefaultTag;

impl VerifyTag for DefaultTag {
    fn tag() -> &'static str { "fenwick_2d_rect_rs_d61" }
}
