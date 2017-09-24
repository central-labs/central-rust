use redis::{Client, Commands, RedisError};
use std::sync::{Arc, Mutex};
use pubsub::Subscriber;
use types::{Identity, Storage, BroadcastStorage, Handle};
use std::collections::HashMap;
use std::thread;
use std::clone::Clone;

pub struct RedisStore {
    parent: &'static str,
    inner: Arc<Mutex<Client>>,
    identity: Identity,
    subscriber: Arc<Mutex<Subscriber>>
}

impl RedisStore {
    
    pub fn create(url: &'static str, parent: &'static str, handlers: HashMap<String, Handle>) 
        -> Result<RedisStore, RedisError> {
        
        let identity : Identity = Identity::default();
        let subscriber = Arc::new(Mutex::new(Subscriber::new(url, parent, identity, handlers)));
        let shared_subscriber = subscriber.clone();
        
        let _ = thread::spawn(move || {
            let subscriber = shared_subscriber.clone();
            let subscriber = &mut *subscriber.lock().unwrap();
            subscriber.call();
        });

        Ok(RedisStore {
            parent: parent,
            inner: Arc::new(Mutex::new(Client::open(url)?)),
            identity: identity,
            subscriber: subscriber
        })
    }
}

impl BroadcastStorage for RedisStore {
    fn publish(&mut self, path: &'static str) {
        let inner = self.inner.clone();
        let inner = &*inner.lock().unwrap();
        let _: Result<String, _> = inner.publish(
            format!("{}:{}", self.parent, path),
            format!("{}", self.identity())
        );
    }
}

impl Storage for RedisStore {
    fn set<Value: Into<String>>(&mut self, path: &'static str, value: Value) {
        {
            let inner = self.inner.clone();
            let inner = &*inner.lock().unwrap();
            
            let _: Result<String, _> = inner.hset(self.parent, path, value.into());    
        }
        
        self.publish(path);
    }

    fn get(&mut self, path: &'static str) -> Result<String, ()> {
        let inner = self.inner.clone();
        let inner = &*inner.lock().unwrap();

        let values: String = inner.hget(self.parent, path).unwrap();
        Ok(values)
    }

    fn identity(&self) -> Identity {
        self.identity.clone()
        // format!("{}:{}", self.identity.0, self.identity.1)
    }

    fn subscriber(&self) -> Arc<Mutex<Subscriber>> {
        self.subscriber.clone()
    }
}
