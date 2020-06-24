// diving-decompression crate
// Written in 2020 by
// Ronald Alonzo <alonzo.ronald@gmail.com>

// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.

//! # Rust diving-decompression library
//!
//! # UNDER CONSTRUCTION

//! ## DO NOT USE THIS PACKAGE UNTIL STABLE VERSION HAS BEEN RELEASED!!

//! ## IMPORTANT NOTE FROM THE AUTHOR
//! this package is under construction, it is **__NOT__** suitable for 
//! usage in real dive operations neither commercial nor recreational, 
//! as we need to make extensive test and audit the package reliability.
//! it is not only a matter of applying unit testing as this calculations
//! are crucial for divers safety; also regardless of the extensive tests 
//! and trials in humans performed by the US Navy along the years with 
//! regards of decompression sickness, it has been stated many times by 
//! relevant stakeholders that these trials do not necessarily entail 100%
//! accuracy on the results of undertaking dive operations within the 
//! constraints of these dive tables. there are many factors that are not 
//! taken into consideration (e.g: water temperature, diver physiological 
//! fitness, unadverted PFOs... to name a few). 

//! This is a library created with the purpose of assisting diving 
//! professionals in planning decompression procedures for air diving
//! operations as per the US Navy diving manual rev7. 
//!
//! It was initially written in TypeScript and then ported to Rust to
//! harness the benefits of a much stronger type system. These safety 
//! guarantees are of crucial importance when dealing with operational 
//! and procedural safety in the commercial diving industry.
//! 
//! This project is and will always be 100% free and open source. 
//! it is open for public review and we welcome PRs as long as they 
//! adhere to international guidelines and acknowledged best practices
//! in the industry, specially those contained within the US Navy dive 
//! manual which is __THE ONLY__ scientifically derived set of guidelines.
//! 
//! Pull Requests based on anecdotical or empirical evidence or those
//! that could contain private parties agendas will always be dismissed
//! by the authors of this project. we do not tolerate private tables 
//! and protocols that aim to distort the good practices in order to 
//! increase allowed diving depth and time limits and shortened decompression
//! procedures with economical purposes.   

// Code conventions
#![forbid(unsafe_code)]
#![deny(non_snake_case)]
#![deny(non_camel_case_types)]
#![deny(non_upper_case_globals)]
#![deny(unused_imports)]
#![deny(unused_mut)]
#![deny(missing_docs)]
#![deny(dead_code)]
#[macro_use]

extern crate serde_derive;
/// this module provides functionality for the US Navy dive tables
pub mod tables;

pub fn no_decompression_limit(depth: u16) -> u16 {
  //! Returns the no decompression limit for any given depth
  //! depth must be expressed in Feet of sea water
  //! No decompression limit is returned in minutes as u16 integer
  
  // no decompression table
  let nodeco_table = tables::nodeco_table()
    .expect("Error serializing the data withthe table deco_table");

  let mut ndl: u16 = 0;  
  for row in nodeco_table.table_data.iter() {
    if row.min_fsw <= depth && depth <= row.max_fsw {
      ndl = row.no_stop_limit
    }
  }
  
  return ndl;
}

pub fn group_letter(dive: tables::types::Dive) -> String {
  //! Returns the group letter for a given dive. 
  //! it takes a struct Dive as a parameter
  //! the depth is expressed in feet of sea water
  //! the bottom_time is expressed in minutes
  //! the group letter is returned as a String
  
  // no decompression table
  let nodeco_table = tables::nodeco_table()
    .expect("Error serializing the data within the deco_table");

  let mut gl: String = String::from("");
  for row in nodeco_table.table_data.iter() {
    if row.min_fsw <= dive.depth && dive.depth <= row.max_fsw {
      for value in row.values.iter() {
        if value.min_time <= dive.bottom_time && dive.bottom_time <= value.max_time {
          gl = String::from(&value.group_letter);
        } 
      }
    }
  }
  
  return gl;
}

pub fn repet_letter(dive_plan: tables::types::DivePlan) -> String {
  //! Returns the repetitive group letter for any given dive. 
  //! it takes the dive profile as a paramater
  //! the depth is expressed in feet of sea water
  //! the time is expressed in minutes
  //! the repetitive group letter is returned as a String
  let nodeco_table = tables::nodeco_table()
    .expect("Error serializing the data within the deco_table");

  let rgl_table = tables::rgl_table()
    .expect("there was an error deserializing deco table");
  
  let mut rl = String::new();
  
  for row in nodeco_table.table_data.iter() {
    if row.min_fsw <= dive_plan.depth && dive_plan.depth <= row.max_fsw {
      for group in row.values.iter() {
        if group.min_time <= dive_plan.bottom_time && dive_plan.bottom_time <= group.max_time {
          for rgl_row in rgl_table.table_data.iter() {
            if rgl_row.group_letter == group.group_letter && rgl_row.min_time <= dive_plan.sit && dive_plan.sit <= rgl_row.max_time {
              rl = String::from(&rgl_row.repet_letter)
            }
          }
        } 
      }
    }
  }
  
  return rl;
}

pub fn residual_nitrogen_time(dive_plan: tables::types::DivePlan) -> u16 {
  //! Returns the residual nitrogen time for a given dive plan. 
  //! it takes the dive profile as a paramater
  //! the depth is expressed in feet of sea water
  //! the time is expressed in minutes
  //! the residual nitrogen time is returned as a u16 integer
  let nodeco_table = tables::nodeco_table()
    .expect("Error deserializing no decompression table");

  let rgl_table = tables::rgl_table()
    .expect("Error deserializing repetitive group letter table");

  let rnt_table = tables::rnt_table()
    .expect("Error deserializing residual nitrogen time table");
  
  let mut rnt = 0;
  
  for row in nodeco_table.table_data.iter() {
    if row.min_fsw <= dive_plan.depth && dive_plan.depth <= row.max_fsw {
      for group in row.values.iter() {
        if group.min_time <= dive_plan.bottom_time && dive_plan.bottom_time <= group.max_time {
          for rgl_row in rgl_table.table_data.iter() {
            if rgl_row.group_letter == group.group_letter && rgl_row.min_time <= dive_plan.sit && dive_plan.sit <= rgl_row.max_time {
              for rnt_column in rnt_table.table_data.iter() {
                if rnt_column.repet_letter == rgl_row.repet_letter {
                  for element in rnt_column.rnt.iter() {
                    if element.min_depth <= dive_plan.next_depth && dive_plan.next_depth <= element.max_depth {
                      rnt = element.rnt
                    } 
                  }
                }
              }
            }
          }
        } 
      }
    }
  }
  
  return rnt;
}

pub fn deco_dive(dive: tables::types::Dive) -> tables::types::RowDeco {
  //! Returns the decompression profile for a given dive. 
  //! it takes a struct Dive as a parameter
  //! the depth is expressed in feet of sea water
  //! the bottom_time is expressed in minutes
  //! the decompression profile is returned as a RowDeco struct
  let deco_table = tables::deco_table()
    .expect("Error deserializing no decompression table");

  let mut deco_profile: tables::types::RowDeco = tables::types::RowDeco {
    min_time: 0,
    max_time: 0,
    air_tat: String::from("0"),
    o2_tat: String::from("0"),
    ttfs: String::from("0"),
    o2cp: 0.0,
    repetgroup_letter: String::from("0"),
    surdo2_recommended: false,
    exceptional_exposure: false,
    surdo2_required: false,
    strict_surdo2: false,
    air_deco_stops: vec![],
    o2_deco_stops: vec![],
  };

  for row_deco in deco_table.table_data.iter() {
    if row_deco.min_fsw <= dive.depth && dive.depth <= row_deco.max_fsw  {
      for profile in row_deco.rows.iter() {
        if profile.min_time <= dive.bottom_time && profile.max_time <= dive.bottom_time {
          deco_profile = profile.clone()
        }
      }
    }
  }

  return deco_profile;
}