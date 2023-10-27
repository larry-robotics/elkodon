pub mod posix_select;

use std::{fmt::Debug, time::Duration};

use elkodon_bb_posix::{
    file_descriptor::FileDescriptor, file_descriptor_set::SynchronousMultiplexing,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactorCreateError {
    UnknownError(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactorAttachError {
    CapacityExceeded,
    UnknownError(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReactorWaitError {
    Interrupt,
    InsufficientPermissions,
    UnknownError,
}

pub trait ReactorGuard<'reactor, 'attachment> {}

pub trait Reactor: Sized {
    type Guard<'reactor, 'attachment>: ReactorGuard<'reactor, 'attachment>
    where
        Self: 'reactor;
    type Builder: ReactorBuilder<Self>;

    fn capacity() -> usize;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn attach<'reactor, 'attachment, F: SynchronousMultiplexing + Debug>(
        &'reactor self,
        value: &'attachment F,
    ) -> Result<Self::Guard<'reactor, 'attachment>, ReactorAttachError>;

    fn try_wait<F: FnMut(&FileDescriptor)>(&self, fn_call: F) -> Result<(), ReactorWaitError>;
    fn timed_wait<F: FnMut(&FileDescriptor)>(
        &self,
        fn_call: F,
        timeout: Duration,
    ) -> Result<(), ReactorWaitError>;
    fn blocking_wait<F: FnMut(&FileDescriptor)>(&self, fn_call: F) -> Result<(), ReactorWaitError>;
}

pub trait ReactorBuilder<T: Reactor> {
    fn new() -> Self;
    fn create(self) -> Result<T, ReactorCreateError>;
}
