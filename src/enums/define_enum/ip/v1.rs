enum IpAddrKind {
    V4,
    V6,
}

pub fn example1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

pub fn example2() {
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_type: IpAddrKind) { }