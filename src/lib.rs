#![feature(fn_traits)]

extern crate redis;
// extern crate rand;

#[macro_use]
extern crate log;
extern crate env_logger;

pub mod types;
pub mod pubsub;
pub mod storage;
pub mod feature;


