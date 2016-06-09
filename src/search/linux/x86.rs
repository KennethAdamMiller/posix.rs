#[repr(C)]
#[derive(Clone)]
pub struct entry {
    pub key: *mut ::schar_t,
    pub data: *mut ::void_t,
}
new!(entry);
