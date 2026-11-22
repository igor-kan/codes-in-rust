pub trait VerifyTag {
    fn tag() -> &'static str;
}

pub struct DefaultTag;

impl VerifyTag for DefaultTag {
    fn tag() -> &'static str { "manacher_oddeven_d57" }
}
