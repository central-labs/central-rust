extern crate central;
#[macro_use]
extern crate log;
extern crate env_logger;

use central::types::{Storage, Handle};
use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use std::convert::Into;

#[derive(Debug)]
pub struct Credential {
    pub user: String,
    pub password: String 
}

impl Into<String> for Credential {
    fn into(self) -> String {
        format!("{}:{}", self.user, self.password)
    }
}

#[test] 
fn test_central_publish() {

    let _ = env_logger::init();
    
    let credential = Credential {
        user: String::from("test"),
        password: String::from("test")
    };

    let mut handlers : HashMap<String, Handle> = HashMap::new();

    handlers.insert(String::from("hello"), Box::new(move |value: Result<String, ()>| {
        assert_eq!(value, Ok(String::from("test:test")));
    }));

    let mut store = central::storage::RedisStore::create("redis://127.0.0.1", "central", handlers).unwrap();
    
    store.set("hello", credential);

    let value = store.get("hello").unwrap();

    assert_eq!(value, "test:test");

    thread::sleep(Duration::from_secs(1));
}
