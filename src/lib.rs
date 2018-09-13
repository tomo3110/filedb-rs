//! filedb is simple file-based data.
//! It becomes a rust version of the golang implementation of the following repository.
//! 
//! https://github.com/matryer/filedb
//! 
//! ## Install
//! 
//! Add the following to your
//! 
//! Cargo.toml
//! ```toml
//! [package]
//! name = "some-project"
//! version = "1.0.0"
//! authors = ["my name <hoge@fuga.com>"]
//! 
//! [dependencies]
//! filedb = "0.1.1"
//! ```
//! 
//! ## Usage
//! 
//! main.rs
//! ```rust
//! extern crate filedb;
//! 
//! use filedb::FileDB;
//! use filedb::callback::*;
//! 
//! fn main() {
//!     let mut db = FileDB::default();
//!     let mut col = match db.c("documents") {
//!         Ok(c) => c.lock().unwrap(),
//!         Err(err) => {
//!             println!("[filedb] failed instance col struct.");
//!             return;
//!         },
//!     };
//!     let res = col.for_each(|index, data| {
//!         println!("index: {}, text: {}", index, String::from_utf8(data).unwrap());
//! 
//!         ForEachResultValue::new(false)
//!     });
//! 
//!     match res {
//!         Ok(_) => println!("[filedb] success!"),
//!         Err(err) => println!("[filedb] errror... {:?}", err),
//!     }
//! }
//! 
//! ```
//! 

#[macro_use]
extern crate failure;
extern crate tempfile;

mod c;
mod db;
mod result;

pub use db::FileDB;
pub use result::*;
pub use c::*;