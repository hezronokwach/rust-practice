use std::net::Ipv4Addr;

fn ips_between(start: &str, end: &str) -> u32 {
    u32::from(end.parse::<Ipv4Addr>().unwrap()) - u32::from(start.parse::<Ipv4Addr>().unwrap())
}