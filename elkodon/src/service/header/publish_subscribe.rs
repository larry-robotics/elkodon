use elkodon_bb_posix::clock::{Time, TimeBuilder};

use crate::port::port_identifiers::UniquePublisherId;

#[derive(Debug)]
#[repr(C)]
struct TimeStamp {
    seconds: u64,
    nanoseconds: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Header {
    publisher_port_id: UniquePublisherId,
    time_stamp: TimeStamp,
}

impl Header {
    pub(crate) fn new(publisher_port_id: UniquePublisherId) -> Self {
        let now = Time::now().unwrap();
        Self {
            publisher_port_id,
            time_stamp: TimeStamp {
                seconds: now.seconds(),
                nanoseconds: now.nanoseconds(),
            },
        }
    }

    pub fn publisher_id(&self) -> UniquePublisherId {
        self.publisher_port_id
    }

    pub fn time_stamp(&self) -> Time {
        TimeBuilder::new()
            .nanoseconds(self.time_stamp.nanoseconds)
            .seconds(self.time_stamp.seconds)
            .create()
    }
}
