#[repr(C)]
#[derive(Clone)]
pub struct entry {
    pub key: *mut ::char_t,
    pub data: *mut ::void_t,
}
new!(entry);
