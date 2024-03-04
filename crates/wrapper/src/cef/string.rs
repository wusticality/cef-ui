use anyhow::{anyhow, Result};
use cef_ui_bindings_linux_x86_64::{cef_string_t, cef_string_utf8_to_utf16};
use std::{
    ffi::c_char,
    fmt::Debug,
    mem::{forget, zeroed},
    slice::from_raw_parts
};

/// Wraps cef_string_t. A cef_string_t is ultimately a
/// typedef for _cef_string_utf16_t, a UTF-16 string.
#[repr(transparent)]
pub struct CefString(cef_string_t);

impl CefString {
    /// Returns a null CefString.
    pub fn null() -> cef_string_t {
        unsafe { zeroed() }
    }

    /// Try and create a CefString from a str.
    pub fn new(s: &str) -> Result<Self> {
        let ret = Self::utf8_to_utf16(s)?;

        Ok(Self(ret))
    }

    /// Try and set the CefString from a str.
    pub fn set(&mut self, s: &str) -> Result<()> {
        self.free();
        self.0 = Self::utf8_to_utf16(s)?;

        Ok(())
    }

    /// Transfers ownership of the pointer.
    pub fn into_raw(self) -> cef_string_t {
        let raw = cef_string_t { ..self.0 };

        forget(self);

        raw
    }

    /// Converts a &str to a cef_string_t.
    fn utf8_to_utf16(s: &str) -> Result<cef_string_t> {
        let mut ret: cef_string_t = unsafe { zeroed() };

        match unsafe { cef_string_utf8_to_utf16(s.as_ptr() as *const c_char, s.len(), &mut ret) } {
            0 => Err(anyhow!("Failed to convert string to UTF-16.")),
            _ => Ok(ret)
        }
    }

    /// Try and free the memory associated with the CefString.
    fn free(&mut self) {
        free_cef_string(&mut self.0);
    }
}

impl Drop for CefString {
    fn drop(&mut self) {
        self.free();
    }
}

impl Debug for CefString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = String::from_utf16_lossy(unsafe { from_raw_parts(self.0.str_, self.0.length) });

        <String as Debug>::fmt(&s, f)
    }
}

/// Try and free a cef_string_t. Don't call this unless you
/// are sure that you know what you are doing or you could
/// potentially cause a double free.
pub fn free_cef_string(s: &mut cef_string_t) {
    if let Some(dtor) = s.dtor {
        unsafe {
            dtor(s.str_);
        }
    }

    *s = unsafe { zeroed() };
}
