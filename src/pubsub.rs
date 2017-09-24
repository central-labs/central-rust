use std::collections::HashMap;
use std::sync::RwLock;
use redis::{Commands, Client};

// use std::sync::atomic::AtomicBool;
use std::sync::Arc;
// use std::thread::{self, JoinHandle};
use types::Handle;

pub struct Subscriber {
    parent: &'static str,
    url: &'static str,
    identity: (u64, u32),
    handlers: Arc<RwLock<HashMap<String, Handle>>>,
}

impl Subscriber {
    pub fn new(url: &'static str, parent: &'static str, identity: (u64, u32), handlers: HashMap<String, Handle>) -> Subscriber {
        Subscriber {
            parent: parent,
            url: url,
            identity: identity,
            handlers: Arc::new(RwLock::new(handlers)),
        }
    }

    // pub fn add(&mut self, namespace: String, handle: Handle) {
    //     let mut handlers = &*self.handlers.clone();
    //     let mut handlers = handlers.write().unwrap();
    //     handlers.insert(namespace, handle);
    // }

    pub fn handlers(&self) -> Arc<RwLock<HashMap<String, Handle>>> {
        self.handlers.clone()
    } 

    pub fn call(&mut self) {
        loop {
            let handlers = self.handlers.read().unwrap();
            let client = Client::open(self.url).unwrap();
            let mut pubsub = client.get_pubsub().unwrap();

            let _ = pubsub.psubscribe(format!("{}:*", self.parent));

            let message = pubsub.get_message().unwrap();
            let payload: String = message.get_payload().unwrap();
            let channel: String = String::from(message.get_channel_name());

            let identity = format!("{}:{}", self.identity.0, self.identity.1);

            fn get(client: Client, parent: &str, key: &str) -> Result<String, ()> {
                let values: String = client.hget(parent, key).unwrap();
                Ok(values)
            }

            if payload != identity {
                debug!("keys: {:?}", handlers.keys());
                let namespace : Vec<&str> = channel.split(':').collect();
                debug!("key : {}", namespace[1]);
                let f = &(*handlers)[namespace[1]];
                f.call((get(client, namespace[0], namespace[1]),));
            }
        }
    }

}
