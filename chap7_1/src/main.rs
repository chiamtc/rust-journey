mod lib;
use restaurant;
//if lib.rs is in the same path as main.rs. lib = the filename
//nesting for import
/*
from

use std::io;
use std::cmp::Ordering;

to */

use std::{io, cmp::Ordering};

/* glob operator -> * <- */
use std::collections::*;

use std::fmt::Result;
use std::io::Result as IoResult;
//as

fn main() {
    println!("Hello, world!");
    println!("{:?}",lib::eat_at_restaurant());
    lib::front_of_house::hosting::add_to_waitlist()
}

//for library import from different file and packages https://stackoverflow.com/questions/45519176/how-do-i-use-or-import-a-local-rust-file
