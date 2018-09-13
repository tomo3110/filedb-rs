# filedb

[![Build Status](https://travis-ci.com/tomo3110/filedb-rs.svg?branch=master)](https://travis-ci.org/tomo3110/filedb-rs)
[![filedb at crates.io](https://img.shields.io/crates/v/filedb.svg)](https://crates.io/crates/filedb)
[![filedb at docs.rs](https://docs.rs/filedb/badge.svg)](https://docs.rs/filedb)

filedb is simple file-based data.  
It becomes a rust version of the golang implementation of the following repository.   

> https://github.com/matryer/filedb

## Install

Add the following to your  

Cargo.toml
```toml
[package]
name = "some-project"
version = "1.0.0"
authors = ["my name <hoge@fuga.com>"]

[dependencies]
filedb = "0.1.1"
```

## Usage

main.rs
```rust
extern crate filedb;

use filedb::FileDB;
use filedb::callback::*;

fn main() {
    let mut db = FileDB::connect("/tmp/db").unwrap();
    let mut col = match db.c("documents") {
        Ok(c) => c.lock().unwrap(),
        Err(err) => {
            println!("[filedb] failed instance col struct.");
            return;
        },
    };
    let res = col.for_each(|index, data| { 
        println!("index: {}, text: {}", index, String::from_utf8(&data).unwrap());

        ForEachResultValue::new(false)
    });

    match res {
        Ok(_) => println!("[filedb] success!"),
        Err(err) => println!("[filedb] errror... {:?}", err),
    }
}

```
