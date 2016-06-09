#[repr(C)]
#[derive(Clone)]
pub struct netent {
    pub n_name: *mut ::schar_t,
    pub n_aliases: *mut *mut ::schar_t,
    pub n_addrtype: ::int_t,
    pub n_net: ::stdint::uint32_t,
}
new!(netent);
#[repr(C)]
#[derive(Clone)]
pub struct hostent {
    pub h_name: *mut ::schar_t,
    pub h_aliases: *mut *mut ::schar_t,
    pub h_addrtype: ::int_t,
    pub h_length: ::int_t,
    pub h_addr_list: *mut *mut ::schar_t,
}
new!(hostent);
#[repr(C)]
#[derive(Clone)]
pub struct servent {
    pub s_name: *mut ::schar_t,
    pub s_aliases: *mut *mut ::schar_t,
    pub s_port: ::int_t,
    pub s_proto: *mut ::schar_t,
}
new!(servent);
#[repr(C)]
#[derive(Clone)]
pub struct protoent {
    pub p_name: *mut ::schar_t,
    pub p_aliases: *mut *mut ::schar_t,
    pub p_proto: ::int_t,
}
new!(protoent);
#[repr(C)]
#[derive(Clone)]
pub struct addrinfo {
    pub ai_flags: ::int_t,
    pub ai_family: ::int_t,
    pub ai_socktype: ::int_t,
    pub ai_protocol: ::int_t,
    pub ai_addrlen: ::sys::socket::socklen_t,
    pub ai_addr: *mut ::sys::socket::sockaddr,
    pub ai_canonname: *mut ::schar_t,
    pub ai_next: *mut addrinfo,
}
new!(addrinfo);
pub const IPPORT_RESERVED: ::int_t = 1024;
pub const AI_PASSIVE: ::int_t = 0x0001;
pub const AI_CANONNAME: ::int_t = 0x0002;
pub const AI_NUMERICHOST: ::int_t = 0x0004;
pub const AI_NUMERICSERV: ::int_t = 0x0400;
pub const AI_V4MAPPED: ::int_t = 0x0008;
pub const AI_ALL: ::int_t = 0x0010;
pub const AI_ADDRCONFIG: ::int_t = 0x0020;
pub const NI_NOFQDN: ::int_t = 4;
pub const NI_NUMERICHOST: ::int_t = 1;
pub const NI_NAMEREQD: ::int_t = 8;
pub const NI_NUMERICSERV: ::int_t = 2;
pub const NI_DGRAM: ::int_t = 16;
pub const EAI_AGAIN: ::int_t = 3;
pub const EAI_BADFLAGS: ::int_t = 1;
pub const EAI_FAIL: ::int_t = 4;
pub const EAI_FAMILY: ::int_t = 6;
pub const EAI_MEMORY: ::int_t = 10;
pub const EAI_NONAME: ::int_t = 2;
pub const EAI_SERVICE: ::int_t = 8;
pub const EAI_SOCKTYPE: ::int_t = 7;
pub const EAI_SYSTEM: ::int_t = 11;
pub const EAI_OVERFLOW: ::int_t = 12;
