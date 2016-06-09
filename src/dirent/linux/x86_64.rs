use std::mem::zeroed;
use std::ptr::copy;

#[repr(C)]
pub struct dirent {
    pub d_ino: ::sys::types::ino_t,
    pub d_off: ::sys::types::off_t,
    pub d_reclen: ::ushort_t,
    pub d_type: ::uchar_t,
    pub d_name: [::char_t; 256usize],
}

impl Clone for dirent {
    fn clone(&self) -> Self {
        let mut d_name : [::char_t; 256usize] = unsafe { zeroed() };
        unsafe { copy(self.d_name.as_ptr(), d_name.as_mut_ptr(), 256usize) };
        return dirent {
            d_ino : self.d_ino,
            d_off : self.d_off,
            d_reclen : self.d_reclen,
            d_type : self.d_type,
            d_name : d_name,
        };
    }

    fn clone_from(&mut self, source: &Self) {
        self.d_ino    = source.d_ino;
        self.d_off    = source.d_off;
        self.d_reclen = source.d_reclen;
        self.d_type   = source.d_type;
        unsafe { copy(source.d_name.as_ptr(), self.d_name.as_mut_ptr(), 256usize); };
    }
}

new!(dirent);

pub type DIR = ::void_t;
