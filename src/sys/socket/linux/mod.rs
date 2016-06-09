pub use self::arch::{socklen_t};
pub use self::arch::{sa_family_t};
pub use self::arch::{SOCK_STREAM};
pub use self::arch::{SOCK_DGRAM};
pub use self::arch::{SOCK_RAW};
pub use self::arch::{SOCK_SEQPACKET};
pub use self::arch::{SOL_SOCKET};
pub use self::arch::{sockaddr};
pub use self::arch::{sockaddr_storage};
pub use self::arch::{MSG_OOB};
pub use self::arch::{MSG_PEEK};
pub use self::arch::{MSG_DONTROUTE};
pub use self::arch::{MSG_CTRUNC};
pub use self::arch::{MSG_TRUNC};
pub use self::arch::{MSG_EOR};
pub use self::arch::{MSG_WAITALL};
pub use self::arch::{MSG_NOSIGNAL};
pub use self::arch::{msghdr};
pub use self::arch::{cmsghdr};
pub use self::arch::{SCM_RIGHTS};
pub use self::arch::{SO_ACCEPTCONN};
pub use self::arch::{SO_BROADCAST};
pub use self::arch::{SO_DEBUG};
pub use self::arch::{SO_DONTROUTE};
pub use self::arch::{SO_ERROR};
pub use self::arch::{SO_KEEPALIVE};
pub use self::arch::{SO_LINGER};
pub use self::arch::{SO_OOBINLINE};
pub use self::arch::{SO_RCVBUF};
pub use self::arch::{SO_RCVLOWAT};
pub use self::arch::{SO_RCVTIMEO};
pub use self::arch::{SO_REUSEADDR};
pub use self::arch::{SO_SNDBUF};
pub use self::arch::{SO_SNDLOWAT};
pub use self::arch::{SO_SNDTIMEO};
pub use self::arch::{SO_TYPE};
pub use self::arch::{SOMAXCONN};
pub use self::arch::{linger};
pub use self::arch::{SHUT_RD};
pub use self::arch::{SHUT_WR};
pub use self::arch::{SHUT_RDWR};
pub use self::arch::{AF_INET};
pub use self::arch::{AF_INET6};
pub use self::arch::{AF_UNIX};
pub use self::arch::{AF_UNSPEC};
pub use self::arch::{CMSG_FIRSTHDR};
pub use self::arch::{CMSG_DATA};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

