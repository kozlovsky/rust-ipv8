//! Payload sent by the receiver of a [PunctureRequestPayload](crate::payloads::puncturerequestpayload::PunctureRequestPayload) to actually puncture a hole  ([NAT puncturing](https://en.wikipedia.org/wiki/UDP_hole_punching))

use crate::payloads::Ipv8Payload;
use serde::{Deserialize, Serialize};
use crate::networking::address::Address;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
/// Payload used for NAT puncturing, containg the addresses which should be used communicate.
pub struct PuncturePayload {
    /// is the lan address of the sender.  Nodes in the same LAN
    /// should use this address to communicate.
    pub lan_walker_address: Address,
    /// is the wan address of the sender.  Nodes not in the same
    /// LAN should use this address to communicate.
    pub wan_walker_address: Address,

    /// is a number that was given in the associated introduction-request.  This
    /// number allows to distinguish between multiple introduction-response messages.
    pub identifier: u16,
}

impl Ipv8Payload for PuncturePayload {
    // doesnt have anything but needed for the default implementation (as of right now)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialization::Packet;
    use std::net::{Ipv4Addr, SocketAddr, IpAddr};

    #[test]
    fn integration_test_creation() {
        let i = PuncturePayload {
            lan_walker_address: Address(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                8000,
            )),
            wan_walker_address: Address(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(42, 42, 42, 42)),
                8000,
            )),
            identifier: 42,
        };

        let mut packet = Packet::new(create_test_header!()).unwrap();
        packet.add(&i).unwrap();

        assert_eq!(
            packet,
            Packet(vec![
                0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 127, 0, 0, 1,
                31, 64, 42, 42, 42, 42, 31, 64, 0, 42,
            ])
        );

        packet.add(&i).unwrap();
        assert_eq!(
            i,
            packet
                .start_deserialize()
                .skip_header()
                .unwrap()
                .next_payload()
                .unwrap()
        );
    }
}
