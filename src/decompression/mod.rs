extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

pub mod types;

pub fn nodeco_table() -> serde_json::Result<types::TableNdl> {
  let mut file = File::open("tables/JSON/usnavy-air-nodeco-rev7.json")
                      .expect("Cant find US NAVY air no-decompression tables");
  
  let mut contents = String::new();

  file.read_to_string(&mut contents)
      .expect("Cant read file!");

  let sertab = serde_json::from_str(&contents); 

  return sertab;
}

pub fn deco_table() -> serde_json::Result<types::TableAirDeco> {
  let mut file = File::open("tables/JSON/usnavy-air-deco-rev7.json")
                      .expect("Cant find US NAVY air decompression tables");
  
  let mut contents = String::new();

  file.read_to_string(&mut contents)
      .expect("Cant read file!");

  let sertab = serde_json::from_str(&contents); 

  return sertab;
}