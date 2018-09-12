# filedb

[![Build Status](https://travis-ci.org/tomo3110/filedb-rs.svg?branch=master)](https://travis-ci.org/tomo3110/filedb-rs)

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
filedb = "0.1.0"
```

## Usage

main.rs
```rust
extern crate filedb;

use filedb::FileDB;
use filedb::callback::*;

fn main() {
    let mut db = FileDB::connect("/tmp/db");
    let res = col.for_each(|index, data| {
        println!("index: {}, text: {}", index, String::from_utf8(&data));

        ForEachResultValue::new(false)
    });

    match res {
        Ok(_) => println!("[filedb] success!"),
        Ok(err) => println!("[filedb] errror... {:?}", err),
    }
}

```
