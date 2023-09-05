/// Modified from https://github.com/rafawo/winutils-rs under MIT license
use std::ffi::c_void;
use windows::core::PWSTR;
use windows::Win32::System::Com::CoTaskMemFree;

/// Thin Rust wrapper of a PWSTR pointer that can be used to
/// receive return parameters from windows API that use CoTaskMemAlloc under the covers.
/// On drop, frees the memory using CoTaskMemFree.
pub struct CoTaskMemWString {
    ptr: PWSTR,
}

impl std::ops::Drop for CoTaskMemWString {
    fn drop(&mut self) {
        unsafe {
            CoTaskMemFree(self.as_option());
        }
    }
}

pub trait AsOption<U> {
    fn as_option(&self) -> Option<U>;
}

impl AsOption<*mut PWSTR> for CoTaskMemWString {
    fn as_option(&self) -> Option<*mut PWSTR> {
        Some(self.ptr.as_ptr() as *mut PWSTR)
    }
}

impl AsOption<*const c_void> for CoTaskMemWString {
    fn as_option(&self) -> Option<*const c_void> {
        Some(self.ptr.as_ptr() as *const c_void)
    }
}

impl CoTaskMemWString {
    /// Creates a new empty CoTaskMemWString, with its pointer initialized to null.
    pub fn new() -> CoTaskMemWString {
        CoTaskMemWString { ptr: PWSTR::null() }
    }

    pub fn as_ptr(&mut self) -> *mut PWSTR {
        &mut self.ptr
    }

    /// This function creates a String representation of the underlying WSTR.
    pub fn to_string(&mut self) -> String {
        match self.ptr {
            ptr if !ptr.is_null() => unsafe { ptr.to_string().unwrap_or(String::from("")) },
            _ => String::from(""),
        }
    }
}
