use libp2p::{Multiaddr, multiaddr::Protocol};
use std::net::{Ipv4Addr, Ipv6Addr};

pub fn parse_listen_address(addr_str: &str) -> Result<Multiaddr, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = addr_str.rsplitn(2, ':').collect();

    if parts.len() != 2 {
        return Err(format!("Invalid address format: {addr_str}").into());
    }

    let ip = parts[1];
    let port: u16 = parts[0].parse()?;

    if let Ok(ip4) = ip.parse::<Ipv4Addr>() {
        Ok(Multiaddr::empty()
            .with(Protocol::Ip4(ip4))
            .with(Protocol::Udp(port))
            .with(Protocol::QuicV1))
    } else if let Ok(ip6) = ip.parse::<Ipv6Addr>() {
        Ok(Multiaddr::empty()
            .with(Protocol::Ip6(ip6))
            .with(Protocol::Udp(port))
            .with(Protocol::QuicV1))
    } else {
        Err(format!("Invalid IP address: {ip}").into())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ipv4_address_parsing() {
        let addr = "127.0.0.1:8082";
        let result = parse_listen_address(addr);
        assert!(result.is_ok());
        let multiaddr = result.unwrap();
        assert!(multiaddr.to_string().contains("/ip4/127.0.0.1/udp/8082/quic-v1"));
    }

    #[tokio::test]
    async fn test_ipv6_address_parsing() {
        let addr = "[::1]:8082";
        let result = parse_listen_address(addr);
        assert!(result.is_ok());
        let multiaddr = result.unwrap();
        assert!(multiaddr.to_string().contains("/ip6/::1/udp/8082/quic-v1"));
    }

    #[tokio::test]
    async fn test_invalid_address_format() {
        let addr = "invalid_address";
        let result = parse_listen_address(addr);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_ipv4_address() {
        let addr = "999.999.999.999:8080";
        let result = parse_listen_address(addr);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_invalid_ipv6_address() {
        let addr = "[gggg::1]:8080";
        let result = parse_listen_address(addr);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_empty_address_list() {
        let addr = "";
        let result = parse_listen_address(addr);
        assert!(result.is_err());
    }

}