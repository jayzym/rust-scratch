enum IpType {
    V4(String),
    V6(String),
}


fn main() {
    let home = IpType:::V4(String::from("127.0.0.1"));
    let loopback = IpType::V6(String::from("::1"));
}
