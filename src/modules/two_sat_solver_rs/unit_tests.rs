#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_2sat_xor_chain() {
    use crate::types::Clause;
    let cs = [Clause{a:0,na:false,b:1,nb:false},Clause{a:0,na:true,b:1,nb:true},Clause{a:0,na:false,b:0,nb:false}];
    let a = crate::core::solve_two_sat(2, &cs).unwrap();
    assert_eq!(a, vec![true, false]);
   
}
