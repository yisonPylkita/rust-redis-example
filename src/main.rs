fn connect() -> redis::Connection {
    redis::Client::open("redis://:@localhost".to_string())
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to connect to Redis")
}

fn main() {
    println!("Hello, world!");

    let mut redis_handle = connect();
    let _: () = redis::cmd("SET")
        .arg("measurand")
        .arg("10")
        .query(&mut redis_handle)
        .expect("failed to execute SET for 'measurand'");
}
