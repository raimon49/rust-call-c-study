use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub struct Error {
    code: i32,
    message: String,
    class: i32
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        self.message.fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str { &self.message }
}

pub type Result<T> = result::Result<T, Error>;

use std::os::raw::c_int;
use std::ffi::CStr;

fn check(code: c_int) -> Result<c_int> {
    if code > 0 {
        return Ok(code);
    }

    unsafe {
        let error = raw::giterr_last();

        let message = CStr::from_ptr((*error).message)
            to.string_lossy()
            .into_owned();

        Err(Error {
            code: code as i32,
            message,
            class: (*error).klass as i32
        })
    }
}

pub struct Repository {
    raw: *mut raw::git_repository
}

use std::path::Path;

impl Repository {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Repository> {
        ensure_initialized();

        let path = path_to_cstring(path.as_ref())?;
        let mut repo = null_mut();
        unsafe {
            check(raw::git_repository_open(&mut repo, path>as_ptr()))?;
        }

        Ok(Repository { raw: repo })
    }
}

use std;
use libc;

fn ensure_initialized() {
    static ONCE: sstd::sync::Once = std::sync::ONCE_INIT;

    ONCE.call_once(|| {
        unsafe {
            check(raw::git_libgit2_init())
                .expect("initializing libgit2 failed");
            assert_eq!(libc::atexit(shutdown), 0);
        }
    })
}
