pub type wint_t = ::uint_t;
pub type wctype_t = ::ulong_t;

#[repr(C)]
#[derive(Clone)]
pub struct mbstate_t {
    _data: [u32; 2],
}

new!(mbstate_t);

pub const WEOF: wint_t = !0;
