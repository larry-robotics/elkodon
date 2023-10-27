#![allow(non_camel_case_types, non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use crate::posix::types::*;

pub unsafe fn sysconf(name: int) -> long {
    crate::internal::sysconf(name)
}

pub unsafe fn pathconf(path: *const char, name: int) -> long {
    crate::internal::pathconf(path, name)
}

pub unsafe fn getpid() -> pid_t {
    crate::internal::getpid()
}

pub unsafe fn getppid() -> pid_t {
    crate::internal::getppid()
}

pub unsafe fn dup(fildes: int) -> int {
    crate::internal::dup(fildes)
}

pub unsafe fn close(fd: int) -> int {
    crate::internal::close(fd)
}

pub unsafe fn read(fd: int, buf: *mut void, count: size_t) -> ssize_t {
    crate::internal::read(fd, buf, count)
}

pub unsafe fn write(fd: int, buf: *const void, count: size_t) -> ssize_t {
    crate::internal::write(fd, buf, count)
}

pub unsafe fn access(pathname: *const char, mode: int) -> int {
    crate::internal::access(pathname, mode)
}

pub unsafe fn unlink(pathname: *const char) -> int {
    crate::internal::unlink(pathname)
}

pub unsafe fn lseek(fd: int, offset: off_t, whence: int) -> off_t {
    crate::internal::lseek(fd, offset, whence)
}

pub unsafe fn getuid() -> uid_t {
    crate::internal::getuid()
}

pub unsafe fn getgid() -> gid_t {
    crate::internal::getgid()
}

pub unsafe fn rmdir(pathname: *const char) -> int {
    crate::internal::rmdir(pathname)
}

pub unsafe fn ftruncate(fd: int, length: off_t) -> int {
    crate::internal::ftruncate(fd, length)
}

pub unsafe fn fchown(fd: int, owner: uid_t, group: gid_t) -> int {
    crate::internal::fchown(fd, owner, group)
}

pub unsafe fn fsync(fd: int) -> int {
    crate::internal::fsync(fd)
}
