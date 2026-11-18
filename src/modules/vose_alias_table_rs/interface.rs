pub trait VerifyTag {
    fn tag() -> &'static str;
}

pub struct DefaultTag;

impl VerifyTag for DefaultTag {
    fn tag() -> &'static str { "vose_alias_table_rs" }
}
