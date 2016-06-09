#[repr(C)]
#[derive(Clone)]
pub struct iovec {
    pub iov_base: *mut ::void_t,
    pub iov_len: ::size_t,
}
new!(iovec);
