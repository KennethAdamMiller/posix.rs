#[repr(C)]
#[derive(Clone)]
pub struct utimbuf {
    pub actime: ::sys::types::time_t,
    pub modtime: ::sys::types::time_t,
}
new!(utimbuf);
