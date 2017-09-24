extern crate central;

use std::collections::HashMap;

fn main() {
    let store = central::storage::RedisStore::create("redis://127.0.0.1", "bukalapak", HashMap::new()).unwrap();
}
