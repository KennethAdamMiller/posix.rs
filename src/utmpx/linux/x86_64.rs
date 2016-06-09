#[derive(Clone)]
#[repr(C)]
struct __exit_status {
    __e_termination: ::short_t,
    __e_exit: ::short_t,
}

#[repr(C)]
pub struct utmpx {
    pub ut_type:          ::short_t,
    pub ut_pid:           ::sys::types::pid_t,
    pub ut_line:          [::char_t; 32usize],
    pub ut_id:            [::char_t; 4usize],
    pub ut_user:          [::char_t; 32usize],
    pub ut_host:          [::char_t; 256usize],
    ut_exit:          __exit_status,
    pub ut_session:       i32,
    pub ut_tv:            ::sys::time::timeval,
    pub ut_addr_v6:       [i32; 4usize],
    __glibc_reserved: [::char_t; 20usize],
}

impl Clone for utmpx {
    fn clone(&self) -> Self {
        return utmpx {
            ut_type:          self.ut_type,
            ut_pid:           self.ut_pid,
            ut_line:          self.ut_line,
            ut_id:            self.ut_id,
            ut_user:          self.ut_user,
            ut_host:          self.ut_host,
            ut_exit:          self.ut_exit.clone(),
            ut_session:       self.ut_session,
            ut_tv:            self.ut_tv.clone(),
            ut_addr_v6:       self.ut_addr_v6,
            __glibc_reserved: self.__glibc_reserved,
        };
    }

    fn clone_from(&mut self, source: &Self) {
        self.ut_type =         source.ut_type;
        self.ut_pid=           source.ut_pid;
        self.ut_line=          source.ut_line;
        self.ut_id=            source.ut_id;
        self.ut_user=          source.ut_user;
        self.ut_host=          source.ut_host;
        self.ut_exit=          source.ut_exit.clone();
        self.ut_session=       source.ut_session;
        self.ut_tv=            source.ut_tv.clone();
        self.ut_addr_v6=       source.ut_addr_v6;
        self.__glibc_reserved= source.__glibc_reserved;
    }
}



new!(utmpx);

pub const EMPTY:         ::short_t = 0;
pub const BOOT_TIME:     ::short_t = 2;
pub const OLD_TIME:      ::short_t = 4;
pub const NEW_TIME:      ::short_t = 3;
pub const USER_PROCESS:  ::short_t = 7;
pub const INIT_PROCESS:  ::short_t = 5;
pub const LOGIN_PROCESS: ::short_t = 6;
pub const DEAD_PROCESS:  ::short_t = 8;
