use std::mem::zeroed;
use std::ptr::copy;

#[repr(C)]
pub struct utsname {
    pub sysname: [::char_t; 65usize],
    pub nodename: [::char_t; 65usize],
    pub release: [::char_t; 65usize],
    pub version: [::char_t; 65usize],
    pub machine: [::char_t; 65usize],
    __domainname: [::char_t; 65usize],
}

impl Clone for utsname {
    fn clone(&self) -> Self { unsafe {
        let mut sysname : [::char_t; 65usize] = zeroed();
        copy(self.sysname.as_ptr(), sysname.as_mut_ptr(), 65usize);
        let mut nodename : [::char_t; 65usize] = zeroed();
        copy(self.nodename.as_ptr(), nodename.as_mut_ptr(), 65usize);
        let mut release : [::char_t; 65usize] = zeroed();
        copy(self.release.as_ptr(), release.as_mut_ptr(), 65usize);
        let mut version : [::char_t; 65usize] = zeroed();
        copy(self.version.as_ptr(), version.as_mut_ptr(), 65usize);
        let mut machine : [::char_t; 65usize] = zeroed();
        copy(self.machine.as_ptr(), machine.as_mut_ptr(), 65usize);
        let mut __domainname : [::char_t; 65usize] = zeroed();
        copy(self.__domainname.as_ptr(), __domainname.as_mut_ptr(), 65usize);
        return utsname {
            sysname: sysname,
            nodename:nodename,
            release: release,
            version: version,
            machine: machine,
            __domainname: __domainname,
        };
    }}

    fn clone_from(&mut self, source: &Self) { unsafe {
        copy(source.sysname.as_ptr(), self.sysname.as_mut_ptr(), 65usize);
        copy(source.nodename.as_ptr(), self.nodename.as_mut_ptr(), 65usize);
        copy(source.release.as_ptr(), self.release.as_mut_ptr(), 65usize);
        copy(source.version.as_ptr(), self.version.as_mut_ptr(), 65usize);
        copy(source.machine.as_ptr(), self.machine.as_mut_ptr(), 65usize);
        copy(source.__domainname.as_ptr(), self.__domainname.as_mut_ptr(), 65usize);
    }}
}


new!(utsname);
