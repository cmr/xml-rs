//#![warn(missing_doc)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![forbid(non_camel_case_types)]
#![feature(collections, io, path, core, std_misc)]

//! This crate currently provides almost XML 1.0/1.1-compliant pull parser.

extern crate core;

#[macro_use]
extern crate bitflags;

pub use reader::EventReader;

pub mod macros;
pub mod name;
pub mod attribute;
pub mod common;
pub mod escape;
pub mod namespace;
pub mod reader;
pub mod writer;
pub mod util;
