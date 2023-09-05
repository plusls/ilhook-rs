use std::io;
use thiserror::Error;

/// Hook errors.
#[derive(Error, Debug)]
pub enum HookError {
    /// Invalid parameter
    #[error("invalid parameter")]
    InvalidParameter,

    /// Error occurs when modifying the memory protect
    #[error("memory protect error, code:{0}")]
    MemoryProtect(u32),

    /// Can't allocate memory
    #[error("memory allocation error, code:{0}")]
    MemoryAllocation(u32),

    /// Can't allocate a memory block between +/-2GB of hooking address
    #[error("searching memory failed")]
    MemorySearching,

    /// Can't get memory layout from /proc/${PID}/maps (only in linux)
    #[error("memory layout format error")]
    MemoryLayoutFormat,

    /// Can't disassemble in the specified address
    #[error("disassemble error")]
    Disassemble,

    /// Can't move code
    #[error("moving code error")]
    MoveCode,

    /// Not supported moving code
    #[error("not supported moving code")]
    MovingCodeNotSupported,

    /// The pre-hook callback failed
    #[error("pre hook failed")]
    PreHook,

    /// Some io error
    #[error("io error")]
    Io(#[from] io::Error),

    /// Unknown error
    #[error("unknown error")]
    Unknown,
}
