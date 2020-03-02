use redis::Connection;

pub fn build(address: &str) -> Connection {
    let client = redis::Client::open(address).unwrap();
    client.get_connection().unwrap()
}
