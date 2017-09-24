use std::sync::{Arc, Mutex};
use pubsub::Subscriber;
use std::convert::TryFrom;
use std::num::ParseIntError;
use std::default::Default;
use rand::{thread_rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::{Eq, PartialEq};
use std::fmt;


pub type Handle = Box<Fn(Result<String, ()>) + Sync + Send>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identity(pub u64, pub u32, pub u32, pub u32);

impl Copy for Identity {}

impl fmt::Display for Identity {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}:{}:{}:{}", self.0, self.1, self.2, self.3)
    }
}

pub enum IdentityErr {
    SizeError,
    DigitError
}

impl From<ParseIntError> for IdentityErr {
    fn from(error: ParseIntError) -> IdentityErr {
        IdentityErr::DigitError
    }
}

impl TryFrom<String> for Identity {
    type Error = IdentityErr;
    fn try_from(s: String) -> Result<Identity, Self::Error> {
        let results : Vec<&str> = s.split(':').collect();
        if results.len() != 4 {
            return Err(IdentityErr::SizeError)
        }

        let first : u64 = results[0].parse::<u64>()?;
        let second : u32 = results[1].parse::<u32>()?;
        let third : u32 = results[2].parse::<u32>()?;
        let fourth : u32 = results[3].parse::<u32>()?;

        Ok(Identity(first, second, third, fourth))
    }
}

impl Default for Identity {
    fn default() -> Self {
        let mut r = thread_rng();
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        Identity(duration.as_secs(), r.next_u32(), r.next_u32(), r.next_u32())
    }
}

pub trait BroadcastStorage {
    fn publish(&mut self, path: &'static str);
}

pub trait Storage {
    fn set<Value: Into<String>>(&mut self, path: &'static str, value: Value);
    fn get(&mut self, path: &'static str) -> Result<String, ()>;
    fn identity(&self) -> Identity;
    fn subscriber(&self) -> Arc<Mutex<Subscriber>>;
}
