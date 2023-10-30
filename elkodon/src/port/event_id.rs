use elkodon_cal::event::TriggerId;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct EventId(u64);

impl EventId {
    pub fn new(value: u64) -> Self {
        EventId(value)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self::new(0)
    }
}

impl TriggerId for EventId {}
