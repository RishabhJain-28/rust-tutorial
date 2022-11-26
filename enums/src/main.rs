enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    println!("Hello, world!");
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let no_number: Option<i32> = None;
}
