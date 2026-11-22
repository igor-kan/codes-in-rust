pub trait VerifyTag {
    fn tag() -> &'static str;
}

pub struct DefaultTag;

impl VerifyTag for DefaultTag {
    fn tag() -> &'static str { "segment_min_point_d57" }
}
