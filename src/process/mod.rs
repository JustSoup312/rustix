//! Process-associated operations.

use crate::imp;

#[cfg(not(target_os = "wasi"))] // WASI doesn't have get[gpu]id.
mod id;
mod sched;
#[cfg(not(target_os = "wasi"))] // WASI doesn't have uname.
mod uname;

#[cfg(not(target_os = "wasi"))]
pub use id::{
    getegid, geteuid, getgid, getpid, getppid, getuid, Gid, Pid, RawGid, RawPid, RawUid, Uid,
};
pub use sched::sched_yield;
#[cfg(not(target_os = "wasi"))]
pub use uname::{uname, Uname};

/// `EXIT_SUCCESS` for use with [`exit`].
///
/// [`exit`]: std::process::exit
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/stdlib.h.html
/// [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html
pub const EXIT_SUCCESS: i32 = imp::process::EXIT_SUCCESS;

/// `EXIT_FAILURE` for use with [`exit`].
///
/// [`exit`]: std::process::exit
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/stdlib.h.html
/// [Linux]: https://man7.org/linux/man-pages/man3/exit.3.html
pub const EXIT_FAILURE: i32 = imp::process::EXIT_FAILURE;

/// The exit status used by a process terminated with `SIGABRT` signal.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://tldp.org/LDP/abs/html/exitcodes.html
#[cfg(not(target_os = "wasi"))]
pub const EXIT_SIGNALED_SIGABRT: i32 = imp::process::EXIT_SIGNALED_SIGABRT;
