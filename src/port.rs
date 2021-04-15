use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

use crate::core::Core;

struct PortInterface {
    inbound: Receiver<i16>,
    outbound: SyncSender<i16>,
}
pub enum Port {
    Dsc,
    Conn(PortInterface),
}

impl Port {
    pub fn new(inbound: Receiver<i16>, outbound: SyncSender<i16>) -> Self {
        Self::Conn(PortInterface { inbound, outbound })
    }

    pub fn write(&self, val: i16) {
        match self {
            Port::Conn(x) => x.outbound.send(val),
            Port::Dsc => loop {
                thread::park();
            },
        };
    }

    pub fn read(&self) -> i16 {
        match self {
            Port::Conn(x) => x.inbound.recv().unwrap(),
            Port::Dsc => loop {
                thread::park();
            },
        }
    }
}
