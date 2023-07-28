use clap::Id;

enum IpAddKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let home = IpAddKind::V4(127, 0, 0, 1);
    let loopback = IpAddKind::V6(String::from("::1"));

    // why cant compile????
    let type = match home {
        V4 => "v4",
        V6 => "v6",
    };

    println!("{}", type);
}