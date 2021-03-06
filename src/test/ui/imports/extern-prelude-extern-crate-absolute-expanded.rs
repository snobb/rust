// compile-pass
// edition:2018

#![feature(extern_crate_item_prelude)]

macro_rules! define_iso { () => {
    extern crate std as iso;
}}

::iso::thread_local! {
    static S: u8 = 0;
}

define_iso!();

fn main() {
    let s = S;
}
