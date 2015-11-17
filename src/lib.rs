#![feature(box_syntax, box_patterns)]
#![feature(rand)]
#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate generic_array;

mod avl;
mod b_plus;
