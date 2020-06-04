#[macro_use]
extern crate serde_derive;
mod tables;

pub fn no_deco_limit(depth: u16) {
  let nodeco = tables::nodeco_table().expect("[ERROR HERE!]");

  println!("{}", nodeco.table_code);
  println!("{}", nodeco.table_name);


}