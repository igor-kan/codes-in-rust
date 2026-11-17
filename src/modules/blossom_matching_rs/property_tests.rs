#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_blossom_sym() {
    let e = [(0,1),(1,2),(2,0),(3,4),(4,5),(5,3),(0,3)];
    let r = crate::core::blossom_match(6, &e);
    assert_eq!(r.pairs, 3);
    for (u, mp) in r.partner.iter().enumerate() {
    if let Some(v) = *mp {
    assert_eq!(r.partner[v], Some(u));
   
}
}
}
