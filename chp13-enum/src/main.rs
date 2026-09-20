// What is Enum?
// A verstaile tool used to represent a type that can take on one of
// several possible variants.

// example is :
// enum IpAddrKind {
//     V4,
//     V6
// }

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let _four = IpAddrKind::V4;
    let _six: IpAddrKind = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Rigth now there's no way of use to store the actual data of IP in the obj.

    // 1. Using structs to do it
    //    Although (2) is better, just showing that it's possible

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 2. Using Enums
    enum IpAddrV2 {
        V4(String),
        V6(String),
    }
    let home: IpAddrV2 = IpAddrV2::V4(String::from("127.0.0.1"));
    let loopback: IpAddrV2 = IpAddrV2::V6(String::from("::1"));

    // Enhanced Enums
    enum IpAddrV3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home: IpAddrV3 = IpAddrV3::V4(127, 0, 0, 1);
    let loopback: IpAddrV3 = IpAddrV3::V6(String::from("::1"));
}
