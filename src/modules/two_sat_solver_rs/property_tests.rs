#![allow(dead_code)]
mod types;
mod errors;
mod interface;
mod core;
mod validation;
mod fast_path;
mod concurrent;
mod invariants;

#[test] fn test_2sat_unsat() {
    use crate::types::Clause;
    let cs = [Clause{a:0,na:false,b:0,nb:false},Clause{a:0,na:true,b:0,nb:true}];
    assert!(crate::core::solve_two_sat(1, &cs).is_none());
   
}
