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

/// single dive object
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Dive {
  /// depth of the dive expressed in feet of sea water
  pub depth: u16, 
  /// bottom time of the dive expressed in minutes
  pub bottom_time: u16,
}

/// single dive plan object
#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct DivePlan {
  /// the depth of the first planned dive expressed in feet of sea water
  pub depth: u16,
  /// the bottom time of the first planned dive expressed in minutes
  pub bottom_time: u16,
  /// the planned surface interval time expressed in minutes
  pub surface_interval_time: u16,
  /// the depth of the next planned dive expressed in feet of sea water
  pub next_dive_depth: u16,
}

impl Dive {
  pub fn new(depth: u16, bottom_time: u16) -> Self {
    //! Instantiate a new Dive object 
    Self {
      depth,
      bottom_time,
    }
  }

  pub fn no_decompression_limit(self) -> u16 {
    //! Returns the no decompression limit of the Dive Object up to a depth of 190 feet of sea water
    //! No decompression limit is returned in minutes as u16 integer

    let mut ndl: u16 = 0;  
    
    if self.depth > 190 {
      return ndl;
    } 

    let nodeco_table = tables::nodeco_table()
      .expect("Error serializing the data withthe table deco_table");

    for row in nodeco_table.table_data.iter() {
      if row.min_fsw <= self.depth && self.depth <= row.max_fsw {
        ndl = row.no_stop_limit
      }
    }

    return ndl;
  }

  pub fn group_letter(self) -> String {
    //! returns the group letter of the Dive object. 
    //! the depth is expressed in feet of sea water
    //! the bottom_time is expressed in minutes
    //! the group letter is returned as a String
    let mut gl: String = String::from("");

    let nodeco_table = tables::nodeco_table()
      .expect("Error serializing the data within the deco_table");

    for row in nodeco_table.table_data.iter() {
      if row.min_fsw <= self.depth && self.depth <= row.max_fsw {
        for value in row.values.iter() {
          if value.min_time <= self.bottom_time && self.bottom_time <= value.max_time {
            gl = String::from(&value.group_letter);
          } 
        }
      }
    }
    
    if gl == String::from("") && self.depth > 0 && self.depth <= 10 && self.bottom_time > 462 {
      gl = String::from("F")
    } else if gl == String::from("") && self.depth > 10 && self.depth <= 15 && self.bottom_time > 449 {
      gl = String::from("I")
    } else if gl == String::from("") && self.depth > 15 && self.depth <= 20 && self.bottom_time > 461 {
      gl = String::from("L")
    }

    if gl == String::from("") && self.depth <= 190 {
      gl = String::from("this dive is out of the time range for no-decompression air dives")
    } else if gl == String::from("") && self.depth > 190 {
      gl = String::from("this dive is out of the depth range for no-decompression air dives")
    }

    return gl;
  }

  pub fn deco_dive(self) -> tables::RowDeco {
    //! returns the decompression profile of the Dive object. 
    //! the depth is expressed in feet of sea water
    //! the bottom_time is expressed in minutes
    //! the decompression profile is returned as a RowDeco struct
    let deco_table = tables::deco_table()
      .expect("Error deserializing no decompression table");

    let mut deco_profile: tables::RowDeco = tables::RowDeco {
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
      if row_deco.min_fsw <= self.depth && self.depth <= row_deco.max_fsw  {
        for profile in row_deco.rows.iter() {
          if profile.min_time <= self.bottom_time && profile.max_time <= self.bottom_time {
            deco_profile = profile.clone()
          }
        }
      }
    }
    return deco_profile;
  }
}

impl DivePlan {
  pub fn new(depth: u16, bottom_time: u16, surface_interval_time: u16, next_dive_depth: u16) -> Self {
    //! Instantiates a new Dive Plan object
    //! the depth and next_dive_depth are expressed in feet of sea water
    //! the bottom_time and surface_interval_time are expressed in minutes
    //! the returned object is of type DivePlan
    Self {
      depth,
      bottom_time,
      surface_interval_time,
      next_dive_depth,
    }
  }

  pub fn from_dive(dive: Dive, surface_interval_time: u16, next_dive_depth: u16) -> Self {
    //! Instantiates a new Dive Plan object from an existing Dive Object
    //! the next_dive_depth is expressed in feet of sea water
    //! the surface_interval_time is expressed in minutes
    //! the returned object is of type DivePlan
    Self {
      depth: dive.depth,
      bottom_time: dive.bottom_time,
      surface_interval_time,
      next_dive_depth,
    }
  }

  pub fn no_decompression_limit(self) -> u16 {
    //! returns the no decompression limit for the first dive of a DivePlan object 
    //! No decompression limit is returned in minutes as u16 integer
    let mut ndl: u16 = 0;  
    
    if self.depth > 190 {
      return ndl;
    } 

    let nodeco_table = tables::nodeco_table()
      .expect("Error serializing the data withthe table deco_table");

    for row in nodeco_table.table_data.iter() {
      if row.min_fsw <= self.depth && self.depth <= row.max_fsw {
        ndl = row.no_stop_limit
      }
    }

    return ndl;
  }

  pub fn group_letter(self) -> String {
    //! returns the group letter of the first dive of a DivePlan object. 
    //! the depth is expressed in feet of sea water
    //! the bottom_time is expressed in minutes
    //! the group letter is returned as a String
    let mut gl: String = String::from("");

    let nodeco_table = tables::nodeco_table()
      .expect("Error serializing the data within the deco_table");

    for row in nodeco_table.table_data.iter() {
      if row.min_fsw <= self.depth && self.depth <= row.max_fsw {
        for value in row.values.iter() {
          if value.min_time <= self.bottom_time && self.bottom_time <= value.max_time {
            gl = String::from(&value.group_letter);
          } 
        }
      }
    }
    
    if gl == String::from("") && self.depth > 0 && self.depth <= 10 && self.bottom_time > 462 {
      gl = String::from("F")
    } else if gl == String::from("") && self.depth > 10 && self.depth <= 15 && self.bottom_time > 449 {
      gl = String::from("I")
    } else if gl == String::from("") && self.depth > 15 && self.depth <= 20 && self.bottom_time > 461 {
      gl = String::from("L")
    }

    if gl == String::from("") && self.depth <= 190 {
      gl = String::from("this dive is out of the time range for no-decompression air dives")
    } else if gl == String::from("") && self.depth > 190 {
      gl = String::from("this dive is out of the depth range for no-decompression air dives")
    }
    return gl;
  }

  pub fn repet_letter(self) -> String {
    //! Returns the repetitive group letter of the DivePlan object. 
    //! the depth and next_dive_depth are expressed in feet of sea water
    //! the bottom_time and surface_interval_time are expressed in minutes
    //! the repetitive group letter is returned as a String
    let nodeco_table = tables::nodeco_table()
      .expect("Error serializing the data within the deco_table");

    let rgl_table = tables::rgl_table()
      .expect("there was an error deserializing deco table");
  
    let mut rl = String::new();
  
    for row in nodeco_table.table_data.iter() {
      if row.min_fsw <= self.depth && self.depth <= row.max_fsw {
        for group in row.values.iter() {
          if group.min_time <= self.bottom_time && self.bottom_time <= group.max_time {
            for rgl_row in rgl_table.table_data.iter() {
              if rgl_row.group_letter == group.group_letter && rgl_row.min_time <= self.surface_interval_time && self.surface_interval_time <= rgl_row.max_time {
                rl = String::from(&rgl_row.repet_letter)
              }
            }
          } 
        }
      }
    }

    return rl;
  }

  pub fn residual_nitrogen_time(self) -> u16 {
    //! Returns the residual nitrogen time of the DivePlan object. 
    //! the depth and next_dive_depth are expressed in feet of sea water
    //! the bottom_time and surface_interval_time are expressed in minutes
    //! the residual nitrogen time is returned as a u16 integer
    let nodeco_table = tables::nodeco_table()
      .expect("Error deserializing no decompression table");

    let rgl_table = tables::rgl_table()
      .expect("Error deserializing repetitive group letter table");

    let rnt_table = tables::rnt_table()
      .expect("Error deserializing residual nitrogen time table");
  
    let mut rnt = 0;
    
    for row in nodeco_table.table_data.iter() {
      if row.min_fsw <= self.depth && self.depth <= row.max_fsw {
        for group in row.values.iter() {
          if group.min_time <= self.bottom_time && self.bottom_time <= group.max_time {
            for rgl_row in rgl_table.table_data.iter() {
              if rgl_row.group_letter == group.group_letter && rgl_row.min_time <= self.surface_interval_time && self.surface_interval_time <= rgl_row.max_time {
                for rnt_column in rnt_table.table_data.iter() {
                  if rnt_column.repet_letter == rgl_row.repet_letter {
                    for element in rnt_column.rnt.iter() {
                      if element.min_depth <= self.next_dive_depth && self.next_dive_depth <= element.max_depth {
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
}
