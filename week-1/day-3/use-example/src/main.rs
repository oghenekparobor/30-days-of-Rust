use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
// // no nested path
// use std::io;
// use std::io::Write

// using nested path
use std::io::{self, Write};

// glob operator * bring all public items into scope
use std::collections:*;