#![feature(proc_macro_hygiene)]
use reign::prelude::*;

fn test() {
    json!(200, status = 100,);
}

fn main() {}
