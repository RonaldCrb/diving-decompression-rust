extern crate serde;
extern crate serde_json;

/// contains the types necesary to deserialize and serialize
/// and operate with the US Navy dive tables
pub mod types;

/// nodeco_table() returns a typed and serialized US Navy air 
/// no-decompression table from rev7 of the US Navy dive manual. 
pub fn nodeco_table() -> serde_json::Result<types::TableNdl> {
  let file = include_str!("JSON/usnavy-air-nodeco-rev7.json");
  let sertab = serde_json::from_str(&file); 
  return sertab;
}

/// deco_table() returns a typed and serialized US Navy air 
/// decompression table from rev7 of the US Navy dive manual
pub fn deco_table() -> serde_json::Result<types::TableAirDeco> {
  let file = include_str!("JSON/usnavy-air-deco-rev7.json");
  let sertab = serde_json::from_str(&file); 
  return sertab;
}

/// rgl_table() returns a typed and serialized US Navy repetitive group letter
/// table from rev7 of the US Navy dive manual
pub fn rgl_table() -> serde_json::Result<types::TableRgl> {
  let file = include_str!("JSON/usnavy-air-repetgroup-rev7.json");
  let sertab = serde_json::from_str(&file); 
  return sertab;
}

/// rnt_table() returns a typed and serialized US Navy residual nitrogen time
/// table from rev7 of the US Navy dive manual
pub fn rnt_table() -> serde_json::Result<types::TableRnt> {
  let file = include_str!("JSON/usnavy-air-rnt-rev7.json");
  let sertab = serde_json::from_str(&file); 
  return sertab;
}