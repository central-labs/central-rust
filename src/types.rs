use std::sync::{Arc, Mutex};
use pubsub::Subscriber;
use std::convert::From;

pub type Handle = Box<Fn(Result<String, ()>) + Sync + Send>;

pub struct Identity(u64, u32, u32, u32);

pub trait BroadcastStorage {
    fn publish(&mut self, path: &'static str);
}

pub trait Storage {
    fn set<Value: Into<String>>(&mut self, path: &'static str, value: Value);
    fn get(&mut self, path: &'static str) -> Result<String, ()>;
    fn identity(&self) -> String;
    fn subscriber(&self) -> Arc<Mutex<Subscriber>>;
}
