extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

/// contains the types necesary to deserialize and serialize
/// and operate with the US Navy dive tables
pub mod types;


/// nodeco_table() returns a typed and serialized US Navy air 
/// no-decompression table from rev7 of the US Navy dive manual. 
pub fn nodeco_table() -> serde_json::Result<types::TableNdl> {
  let mut file = File::open("./JSON/usnavy-air-nodeco-rev7.json")
  .expect("Cant find US NAVY air no-decompression tables");
  
  let mut contents = String::new();

  file.read_to_string(&mut contents)
      .expect("Cant read file!");

  let sertab = serde_json::from_str(&contents); 

  return sertab;
}

/// deco_table() returns a typed and serialized US Navy air 
/// decompression table from rev7 of the US Navy dive manual
pub fn deco_table() -> serde_json::Result<types::TableAirDeco> {
  let mut file = File::open("./JSON/usnavy-air-deco-rev7.json")
                      .expect("Cant find US NAVY air decompression tables");
  
  let mut contents = String::new();

  file.read_to_string(&mut contents)
      .expect("Cant read file!");

  let sertab = serde_json::from_str(&contents); 

  return sertab;
}

/// rgl_table() returns a typed and serialized US Navy repetitive group letter
/// table from rev7 of the US Navy dive manual
pub fn rgl_table() -> serde_json::Result<types::TableRgl> {
  let mut file = File::open("./JSON/usnavy-air-repetgroup-rev7.json")
                      .expect("Cant find US NAVY repetitive group letter table");
  
  let mut contents = String::new();

  file.read_to_string(&mut contents)
      .expect("Cant read file!");

  let sertab = serde_json::from_str(&contents); 

  return sertab;
}

/// rnt_table() returns a typed and serialized US Navy residual nitrogen time
/// table from rev7 of the US Navy dive manual
pub fn rnt_table() -> serde_json::Result<types::TableRnt> {
  let mut file = File::open("./JSON/usnavy-air-rnt-rev7.json")
                      .expect("Cant find US NAVY residual nitrogen time table");
  
  let mut contents = String::new();

  file.read_to_string(&mut contents)
      .expect("Cant read file!");

  let sertab = serde_json::from_str(&contents); 

  return sertab;
}