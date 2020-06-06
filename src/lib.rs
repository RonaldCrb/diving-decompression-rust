// Rust diving-decompression library
// Written in 2020 by
// Ronald Alonzo <alonzo.ronald@gmail.com>
//
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
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#[macro_use]

extern crate serde_derive;
pub mod tables;