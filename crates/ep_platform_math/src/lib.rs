//! Safe scalar math transport to the same CRT used by the pinned Native build.
//!
//! Windows GNU's default CRT differs from that build's Universal CRT. Only
//! explicitly called inverse-trig, exponential and Blind scalar operations use the System32 UCRT provider.
//! Other targets retain Rust's ordinary scalar methods. No engine is loaded.

use std::fmt::{Display, Formatter};

/// Failure to resolve the required platform math provider; no numeric fallback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MathProviderError {
    stage: &'static str,
    win32_code: u32,
}

impl Display for MathProviderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "System32 UCRT math provider {} failed (Win32 {})",
            self.stage, self.win32_code
        )
    }
}
impl std::error::Error for MathProviderError {}

/// Evaluate arcsine using the selected platform's actual scalar math provider.
pub fn asin(value: f64) -> Result<f64, MathProviderError> {
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    {
        windows_gnu::asin(value)
    }
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    {
        Ok(value.asin())
    }
}

/// Evaluate arccosine using the selected platform's actual scalar math provider.
pub fn acos(value: f64) -> Result<f64, MathProviderError> {
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    {
        windows_gnu::acos(value)
    }
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    {
        Ok(value.acos())
    }
}

/// Evaluate exp using the selected platform's actual scalar math provider.
pub fn exp(value: f64) -> Result<f64, MathProviderError> {
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    {
        windows_gnu::exp(value)
    }
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    {
        Ok(value.exp())
    }
}

/// Evaluate sine using the selected platform's actual scalar math provider.
pub fn sin(value: f64) -> Result<f64, MathProviderError> {
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    {
        windows_gnu::sin(value)
    }
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    {
        Ok(value.sin())
    }
}

/// Evaluate cosine using the selected platform's actual scalar math provider.
pub fn cos(value: f64) -> Result<f64, MathProviderError> {
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    {
        windows_gnu::cos(value)
    }
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    {
        Ok(value.cos())
    }
}

/// Evaluate square root using the selected platform's actual scalar math provider.
pub fn sqrt(value: f64) -> Result<f64, MathProviderError> {
    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    {
        windows_gnu::sqrt(value)
    }
    #[cfg(not(all(target_os = "windows", target_env = "gnu")))]
    {
        Ok(value.sqrt())
    }
}

// This crate does not inherit the workspace forbid policy. Its own deny policy
// permits unsafe code only inside this reviewed OS/CRT boundary. All exported
// APIs above are safe, and ep_runtime keeps its inherited unsafe-code forbid.
#[cfg(all(target_os = "windows", target_env = "gnu"))]
#[allow(
    unsafe_code,
    reason = "Private fixed System32 CRT loader and scalar C ABI calls only"
)]
mod windows_gnu {
    use super::MathProviderError;
    use std::{ffi::c_void, sync::OnceLock};

    type Unary = unsafe extern "C" fn(f64) -> f64;
    static MODULE: OnceLock<Result<usize, MathProviderError>> = OnceLock::new();
    static ASIN: OnceLock<Result<Unary, MathProviderError>> = OnceLock::new();
    static ACOS: OnceLock<Result<Unary, MathProviderError>> = OnceLock::new();
    static EXP: OnceLock<Result<Unary, MathProviderError>> = OnceLock::new();
    static SIN: OnceLock<Result<Unary, MathProviderError>> = OnceLock::new();
    static COS: OnceLock<Result<Unary, MathProviderError>> = OnceLock::new();
    static SQRT: OnceLock<Result<Unary, MathProviderError>> = OnceLock::new();

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn LoadLibraryExW(name: *const u16, file: *mut c_void, flags: u32) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
        fn GetLastError() -> u32;
    }

    fn module() -> Result<usize, MathProviderError> {
        *MODULE.get_or_init(|| {
            let name: Vec<u16> = "ucrtbase.dll"
                .encode_utf16()
                .chain(std::iter::once(0))
                .collect();
            // SAFETY: name is NUL-terminated UTF16 and lives through this call;
            // the null file handle and documented flag restrict lookup to the
            // OS System32 provider. The module reference is intentionally kept
            // for process lifetime, so cached code pointers never outlive it.
            let handle = unsafe { LoadLibraryExW(name.as_ptr(), std::ptr::null_mut(), 0x800) };
            if handle.is_null() {
                // SAFETY: GetLastError has no pointer arguments or preconditions.
                return Err(MathProviderError {
                    stage: "LoadLibraryExW(ucrtbase.dll)",
                    win32_code: unsafe { GetLastError() },
                });
            }
            Ok(handle as usize)
        })
    }

    fn resolve(
        cache: &OnceLock<Result<Unary, MathProviderError>>,
        name: &'static [u8],
        stage: &'static str,
    ) -> Result<Unary, MathProviderError> {
        *cache.get_or_init(|| {
            let handle = module()? as *mut c_void;
            // SAFETY: handle is the retained successful System32 module above;
            // the only callers provide the fixed NUL-terminated asin/acos/exp/sin/cos/sqrt names.
            let symbol = unsafe { GetProcAddress(handle, name.as_ptr()) };
            if symbol.is_null() {
                // SAFETY: same argument-free OS error accessor as above.
                return Err(MathProviderError {
                    stage,
                    win32_code: unsafe { GetLastError() },
                });
            }
            // SAFETY: the System32 UCRT exports asin, acos, exp, sin, cos and sqrt with the documented
            // C ABI double(double), and a Windows function pointer has this size.
            // No arbitrary symbol name, library or signature reaches this helper.
            Ok(unsafe { std::mem::transmute::<*mut c_void, Unary>(symbol) })
        })
    }

    pub(super) fn asin(value: f64) -> Result<f64, MathProviderError> {
        let function = resolve(&ASIN, b"asin\0", "GetProcAddress(asin)")?;
        // SAFETY: resolve returns that exact retained C ABI scalar function.
        Ok(unsafe { function(value) })
    }
    pub(super) fn acos(value: f64) -> Result<f64, MathProviderError> {
        let function = resolve(&ACOS, b"acos\0", "GetProcAddress(acos)")?;
        // SAFETY: resolve returns that exact retained C ABI scalar function.
        Ok(unsafe { function(value) })
    }
    pub(super) fn exp(value: f64) -> Result<f64, MathProviderError> {
        let function = resolve(&EXP, b"exp\0", "GetProcAddress(exp)")?;
        // SAFETY: resolve returns that exact retained C ABI scalar function.
        Ok(unsafe { function(value) })
    }
    pub(super) fn sin(value: f64) -> Result<f64, MathProviderError> {
        let function = resolve(&SIN, b"sin\0", "GetProcAddress(sin)")?;
        // SAFETY: resolve returns that exact retained C ABI scalar function.
        Ok(unsafe { function(value) })
    }
    pub(super) fn cos(value: f64) -> Result<f64, MathProviderError> {
        let function = resolve(&COS, b"cos\0", "GetProcAddress(cos)")?;
        // SAFETY: resolve returns that exact retained C ABI scalar function.
        Ok(unsafe { function(value) })
    }
    pub(super) fn sqrt(value: f64) -> Result<f64, MathProviderError> {
        let function = resolve(&SQRT, b"sqrt\0", "GetProcAddress(sqrt)")?;
        // SAFETY: resolve returns that exact retained C ABI scalar function.
        Ok(unsafe { function(value) })
    }
}
