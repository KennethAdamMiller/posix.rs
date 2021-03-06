pub use self::os::{E2BIG};
pub use self::os::{EACCES};
pub use self::os::{EADDRINUSE};
pub use self::os::{EADDRNOTAVAIL};
pub use self::os::{EAFNOSUPPORT};
pub use self::os::{EAGAIN};
pub use self::os::{EALREADY};
pub use self::os::{EBADF};
pub use self::os::{EBADMSG};
pub use self::os::{EBUSY};
pub use self::os::{ECANCELED};
pub use self::os::{ECHILD};
pub use self::os::{ECONNABORTED};
pub use self::os::{ECONNREFUSED};
pub use self::os::{ECONNRESET};
pub use self::os::{EDEADLK};
pub use self::os::{EDESTADDRREQ};
pub use self::os::{EDOM};
pub use self::os::{EDQUOT};
pub use self::os::{EEXIST};
pub use self::os::{EFAULT};
pub use self::os::{EFBIG};
pub use self::os::{EHOSTUNREACH};
pub use self::os::{EIDRM};
pub use self::os::{EILSEQ};
pub use self::os::{EINPROGRESS};
pub use self::os::{EINTR};
pub use self::os::{EINVAL};
pub use self::os::{EIO};
pub use self::os::{EISCONN};
pub use self::os::{EISDIR};
pub use self::os::{ELOOP};
pub use self::os::{EMFILE};
pub use self::os::{EMLINK};
pub use self::os::{EMSGSIZE};
pub use self::os::{EMULTIHOP};
pub use self::os::{ENAMETOOLONG};
pub use self::os::{ENETDOWN};
pub use self::os::{ENETRESET};
pub use self::os::{ENETUNREACH};
pub use self::os::{ENFILE};
pub use self::os::{ENOBUFS};
pub use self::os::{ENODATA};
pub use self::os::{ENODEV};
pub use self::os::{ENOENT};
pub use self::os::{ENOEXEC};
pub use self::os::{ENOLCK};
pub use self::os::{ENOLINK};
pub use self::os::{ENOMEM};
pub use self::os::{ENOMSG};
pub use self::os::{ENOPROTOOPT};
pub use self::os::{ENOSPC};
pub use self::os::{ENOSR};
pub use self::os::{ENOSTR};
pub use self::os::{ENOSYS};
pub use self::os::{ENOTCONN};
pub use self::os::{ENOTDIR};
pub use self::os::{ENOTEMPTY};
pub use self::os::{ENOTRECOVERABLE};
pub use self::os::{ENOTSOCK};
pub use self::os::{ENOTSUP};
pub use self::os::{ENOTTY};
pub use self::os::{ENXIO};
pub use self::os::{EOPNOTSUPP};
pub use self::os::{EOVERFLOW};
pub use self::os::{EOWNERDEAD};
pub use self::os::{EPERM};
pub use self::os::{EPIPE};
pub use self::os::{EPROTO};
pub use self::os::{EPROTONOSUPPORT};
pub use self::os::{EPROTOTYPE};
pub use self::os::{ERANGE};
pub use self::os::{EROFS};
pub use self::os::{ESPIPE};
pub use self::os::{ESRCH};
pub use self::os::{ESTALE};
pub use self::os::{ETIME};
pub use self::os::{ETIMEDOUT};
pub use self::os::{ETXTBSY};
pub use self::os::{EWOULDBLOCK};
pub use self::os::{EXDEV};
pub use self::os::{errno};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;
