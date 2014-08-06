pub static PTHREAD_BARRIER_SERIAL_THREAD: ::int_t = 1;
pub static PTHREAD_CANCEL_ASYNCHRONOUS: ::int_t = 1;
pub static PTHREAD_CANCEL_ENABLE: ::int_t = 0;
pub static PTHREAD_CANCEL_DEFERRED: ::int_t = 0;
pub static PTHREAD_CANCEL_DISABLE: ::int_t = 1;
pub static PTHREAD_CREATE_DETACHED: ::int_t = 1;
pub static PTHREAD_CREATE_JOINABLE: ::int_t = 0;
pub static PTHREAD_EXPLICIT_SCHED: ::int_t = 1;
pub static PTHREAD_INHERIT_SCHED: ::int_t = 0;
pub static PTHREAD_MUTEX_DEFAULT: ::int_t = 0;
pub static PTHREAD_MUTEX_ERRORCHECK: ::int_t = 2;
pub static PTHREAD_MUTEX_NORMAL: ::int_t = 0;
pub static PTHREAD_MUTEX_RECURSIVE: ::int_t = 1;
pub static PTHREAD_MUTEX_ROBUST: ::int_t = 1;
pub static PTHREAD_MUTEX_STALLED: ::int_t = 0;
pub static PTHREAD_ONCE_INIT: ::int_t = 0;
pub static PTHREAD_PRIO_INHERIT: ::int_t = 1;
pub static PTHREAD_PRIO_NONE: ::int_t = 0;
pub static PTHREAD_PRIO_PROTECT: ::int_t = 2;
pub static PTHREAD_PROCESS_SHARED: ::int_t = 1;
pub static PTHREAD_PROCESS_PRIVATE: ::int_t = 0;
pub static PTHREAD_SCOPE_PROCESS: ::int_t = 1;
pub static PTHREAD_SCOPE_SYSTEM: ::int_t = 0;

pub static PTHREAD_CANCELED: *mut ::void_t = -1 as *mut _;

pub fn PTHREAD_COND_INITIALIZER() -> ::sys::types::pthread_cond_t {
    unsafe { ::std::mem::zeroed() }
}

pub fn PTHREAD_MUTEX_INITIALIZER() -> ::sys::types::pthread_mutex_t {
    unsafe { ::std::mem::zeroed() }
}

pub fn PTHREAD_RWLOCK_INITIALIZER() -> ::sys::types::pthread_rwlock_t {
    unsafe { ::std::mem::zeroed() }
}