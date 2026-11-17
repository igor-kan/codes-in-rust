pub trait VerifyTag {
    fn tag() -> &'static str;
}

pub struct DefaultTag;

impl VerifyTag for DefaultTag {
    fn tag() -> &'static str { "xor_basis_gf2_rs" }
}
