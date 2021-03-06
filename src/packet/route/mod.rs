//! RTNETLINK aka Netlink Route Family is used for network device configuration.
//!
//! Different layer operations are implemented as traits 
//! on NetlinkConnection
use packet::netlink::{MutableNetlinkPacket,NetlinkPacket};
use packet::netlink::{NLM_F_REQUEST, NLM_F_DUMP};
use packet::netlink::{NLMSG_NOOP,NLMSG_ERROR,NLMSG_DONE,NLMSG_OVERRUN};
use packet::netlink::{NetlinkBufIterator};
use ::socket::{NetlinkSocket,NetlinkProtocol};
use packet::netlink::NetlinkConnection;
use pnet::packet::MutablePacket;
use pnet::packet::Packet;
use pnet::packet::PacketSize;
use libc;
use std::io::Read;

pub mod addr;
pub mod link;
pub mod neighbour;
pub mod route;
pub mod rule;

include!(concat!(env!("OUT_DIR"), "/route/route.rs"));

const RTA_ALIGNTO: usize = 4;

fn align(len: usize) -> usize {
    ((len)+RTA_ALIGNTO-1) & !(RTA_ALIGNTO-1)
}

/// RTNETLINK attribute iterator
pub struct RtAttrIterator<'a> {
    buf: &'a [u8],
}

impl<'a> RtAttrIterator<'a> {
    fn new(buf: &'a [u8]) -> Self {
        RtAttrIterator {
            buf: buf,
        }
    }
}

impl<'a> Iterator for RtAttrIterator<'a> {
    type Item = RtAttrPacket<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(rta) = RtAttrPacket::new(&self.buf[..]) {
            let len = rta.get_rta_len() as usize; 
            if len < 4 {
                return None;
            }
            self.buf = &self.buf[align(len as usize)..];
            return Some(rta);
        }
        None
    }
}


