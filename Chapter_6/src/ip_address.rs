pub struct Ipv4Addr {
    kind: IpAddrKind,
    address: (u8,u8,u8,u8),
}

pub struct Ipv6Addr {
    kind: IpAddrKing,
    address: String,
}

pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

