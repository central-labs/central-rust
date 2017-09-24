use redis::Client;

pub trait Feature: Fn(Client) + Send + Sync {
    fn call(cl: Client);
}
