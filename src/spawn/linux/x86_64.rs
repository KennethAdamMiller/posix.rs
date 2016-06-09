use std::mem::zeroed;
use std::ptr::copy;

#[repr(C)]
pub struct posix_spawnattr_t {
    _data: [u64; 42],
}

impl Clone for posix_spawnattr_t {
    fn clone(&self) -> Self {
        let mut _data : [u64; 42] = unsafe { zeroed() };
        unsafe { copy(self._data.as_ptr(), _data.as_mut_ptr(), 42); };
        return posix_spawnattr_t {
            _data : _data,
        };
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe { copy(source._data.as_ptr(), self._data.as_mut_ptr(), 42); }
    }
}

new!(posix_spawnattr_t);

#[repr(C)]
pub struct posix_spawn_file_actions_t {
    _data: [u64; 10],
}

impl Clone for posix_spawn_file_actions_t {
    fn clone(&self) -> Self {
        let mut _data : [u64; 10] = unsafe { zeroed() };
        unsafe { copy(self._data.as_ptr(), _data.as_mut_ptr(), 10); };
        return posix_spawn_file_actions_t {
            _data : _data,
        };
    }

    fn clone_from(&mut self, source: &Self) {
        unsafe { copy(source._data.as_ptr(), self._data.as_mut_ptr(), 10); };
    }
}

new!(posix_spawn_file_actions_t);

pub const POSIX_SPAWN_RESETIDS:      ::short_t = 1;
pub const POSIX_SPAWN_SETPGROUP:     ::short_t = 2;
pub const POSIX_SPAWN_SETSCHEDPARAM: ::short_t = 16;
pub const POSIX_SPAWN_SETSCHEDULER:  ::short_t = 32;
pub const POSIX_SPAWN_SETSIGDEF:     ::short_t = 4;
pub const POSIX_SPAWN_SETSIGMASK:    ::short_t = 8;
