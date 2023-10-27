#[repr(C)]
pub(crate) struct Message<Header, Data> {
    pub(crate) header: Header,
    pub(crate) data: Data,
}
