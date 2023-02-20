#![cfg(target_pointer_width = "64")]

use std::mem;
use syn::{Expr, Item, Lit, Pat, Type};

#[test]
fn test_expr_size() {
    if mem::size_of::<Expr>() != 280 {
        assert_eq! (mem::size_of::<Expr>(), 272);
    }
}

#[test]
fn test_item_size() {
    if mem::size_of::<Item>() != 344 {
        assert_eq!(mem::size_of::<Item>(), 320);
    }
}

#[test]
fn test_type_size() {
    if mem::size_of::<Type>() != 304 {
        assert_eq!(mem::size_of::<Type>(), 288);
    }
}

#[test]
fn test_pat_size() {
    assert_eq!(mem::size_of::<Pat>(), 144);
}

#[test]
fn test_lit_size() {
    if mem::size_of::<Lit>() != 40 {
        assert_eq!(mem::size_of::<Lit>(), 32);
    }
}
