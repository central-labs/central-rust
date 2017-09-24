use redis::{Client, Commands};
use std::sync::{Arc, Mutex};
use pubsub::Subscriber;
use types::{Storage, BroadcastStorage, Handle};
use std::collections::HashMap;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct RedisStore {
    parent: &'static str,
    inner: Arc<Mutex<Client>>,
    identity: (u64, u32),
    subscriber: Arc<Mutex<Subscriber>>
}

impl RedisStore {
    
    pub fn create(url: &'static str, parent: &'static str, handlers: HashMap<String, Handle>) -> RedisStore {
        
        let identity : (u64, u32) = {
            let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            (duration.as_secs(), duration.subsec_nanos())
        };

        let subscriber = Arc::new(Mutex::new(Subscriber::new(url, parent, identity, handlers)));
        let shared_subscriber = subscriber.clone();
        
        let _ = thread::spawn(move || {
            let subscriber = shared_subscriber.clone();
            let subscriber = &mut *subscriber.lock().unwrap();
            subscriber.call();
        });

        let store = RedisStore {
            parent: parent,
            inner: Arc::new(Mutex::new(Client::open(url).unwrap())),
            identity: identity,
            subscriber: subscriber
        };

        store
    }
}

impl BroadcastStorage for RedisStore {
    fn publish(&mut self, path: &'static str) {
        let inner = self.inner.clone();
        let inner = &*inner.lock().unwrap();
        let _: Result<String, _> = inner.publish(
            format!("{}:{}", self.parent, path),
            self.identity(),
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

    fn identity(&self) -> String {
        format!("{}:{}", self.identity.0, self.identity.1)
    }

    fn subscriber(&self) -> Arc<Mutex<Subscriber>> {
        self.subscriber.clone()
    }
}
