use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;

use crate::core::Core;

struct PortInstance {
    inbound: Receiver<i16>,
    outbound: SyncSender<i16>,
}

pub enum Port {
    Broken,
    Ok(PortInstance),
}

impl Port {
    pub fn new(inbound: Receiver<i16>, outbound: SyncSender<i16>) -> Self {
        Self::Ok(PortInstance { inbound, outbound })
    }

    pub fn send(&self, val: i16) {
        match self {
            Port::Ok(x) => x.outbound.send(val),
            Port::Broken => loop {
                thread::park();
            },
        };
    }

    pub fn get(&self) -> i16 {
        match self {
            Port::Ok(x) => x.inbound.recv().unwrap(),
            Port::Broken => loop {
                thread::park();
            },
        }
    }
}

impl Default for Port {
    fn default() -> Self {
        Port::Broken
    }
}

pub fn bind_up_down(north: &mut Core, south: &mut Core) {
    let (tx_a, rx_a) = sync_channel(1);
    let (tx_b, rx_b) = sync_channel(1);
    north.down = Port::new(rx_a, tx_b);
    south.up = Port::new(rx_b, tx_a);
}

pub fn bind_left_right(west: &mut Core, east: &mut Core) {
    let (tx_a, rx_a) = sync_channel(1);
    let (tx_b, rx_b) = sync_channel(1);
    west.right = Port::new(rx_a, tx_b);
    east.left = Port::new(rx_b, tx_a);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::instructions::mut_instructions::MutInstr;
    use crate::resource::SrcType;
    use crate::{instructions::mut_instructions::Mut, resource::DestType};

    #[test]
    fn it_should_bind_vertically() {
        let mut core_a = Core::new();
        let mut core_b = Core::new();

        bind_up_down(&mut core_a, &mut core_b);

        let move_down = Mut::new_mov(SrcType::Literal(16), DestType::Down);
        let add_up = Mut::new_add(SrcType::Resource(DestType::Up));
        move_down.execute(&mut core_a);
        add_up.execute(&mut core_b);

        assert_eq!(core_b.acc, 16);
    }

    #[test]
    fn it_should_bind_horizontally() {
        let mut core_a = Core::new();
        let mut core_b = Core::new();

        bind_left_right(&mut core_a, &mut core_b);

        let move_right = Mut::new_mov(SrcType::Literal(16), DestType::Right);
        let add_left = Mut::new_add(SrcType::Resource(DestType::Left));
        move_right.execute(&mut core_a);
        add_left.execute(&mut core_b);

        assert_eq!(core_b.acc, 16);
    }
}
