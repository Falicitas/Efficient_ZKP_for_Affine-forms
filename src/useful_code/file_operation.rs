use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fmt::format;
use std::fs::{self, File};
use std::io::prelude::*;
#[derive(Debug, Serialize, Deserialize)]
pub struct Q {
    size: u64,
    name: String,
}

fn main() {
    let path = "hello_rust.txt";

    fs::write(
        path,
        serde_json::to_string(&Q {
            size: 32,
            name: String::from("hello"),
        })
        .unwrap(),
    )
    .unwrap();

    //one json file, one json data

    let p: Q = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();

    println!("{}", p.name);

    // use std::fs::OpenOptions;
    // let mut file = OpenOptions::new()
    //         .read(true)
    //         .write(true)
    //         .create(true)
    //         .append(true)// append after file
    //         .open(path).unwrap();
    // file.write_all(txt.as_bytes());
}
