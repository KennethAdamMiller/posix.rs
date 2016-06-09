use std::mem::zeroed;
use std::ptr::copy;

#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: ::sys::socket::sa_family_t,
    pub sun_path: [::char_t; 108usize],
}

impl Clone for sockaddr_un {
    fn clone(&self) -> Self {
        let mut sun_path : [::char_t; 108usize] = unsafe { zeroed() };
        unsafe { copy(self.sun_path.as_ptr(), sun_path.as_mut_ptr(), 108usize); };
        return sockaddr_un {
            sun_family : self.sun_family,
            sun_path : sun_path,
        };
    }

    fn clone_from(&mut self, source: &Self) {
        self.sun_family = source.sun_family;
        unsafe { copy(source.sun_path.as_ptr(), self.sun_path.as_mut_ptr(), 108usize); };
    }
}


impl ::AsSlice for sockaddr_un { }
impl ::AsMutSlice for sockaddr_un { }

new!(sockaddr_un);
