use core::fmt;

#[repr(C)]
pub(crate) struct Message<Header, Data> {
    pub(crate) header: Header,
    pub(crate) data: Data,
}

impl<Header: fmt::Debug, Data: fmt::Debug> fmt::Debug for Message<Header, Data> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Message<Header, Data>")
            .field("header", &self.header)
            .field("data", &self.data)
            .finish()
    }
}
