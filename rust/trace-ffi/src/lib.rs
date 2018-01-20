extern crate libc;
extern crate xi_trace;

use libc::c_char;
use std::ffi::CStr;

#[derive(Debug)]
enum ConversionError {
    NullPointer,
    Encoding(std::str::Utf8Error),
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            ConversionError::NullPointer => write!(f, "NullPointer"),
            ConversionError::Encoding(err) => write!(f, "Encoding({})", err)
        }
    }
}

fn c_from_str(c_str: *const c_char) -> Result<String, ConversionError> {
    if c_str.is_null() {
        Err(ConversionError::NullPointer)
    } else {
        unsafe {
            CStr::from_ptr(c_str).to_str()
                .map_err(|e| ConversionError::Encoding(e))
                .map(|s| s.to_string())
        }
    }
}

fn c_from_categories(c_categories: *mut *const c_char)
                     -> Result<Vec<String>, ConversionError>
{
    if c_categories.is_null() {
        return Err(ConversionError::NullPointer);
    }

    unsafe {
        let mut categories = Vec::new();
        while !(*c_categories).is_null() {
            let category = c_from_str(*c_categories);
            if category.is_err() {
                return Err(category.unwrap_err());
            }
            categories.push(category.unwrap());
        }
        return Ok(categories);
    }
}

/// Disable tracing & discard all sample data.  See `xi_trace::disable_tracing`.
/// All samples attempting to record after this function call will also be
/// discarded.  The default is for tracing to be disabled.
#[no_mangle]
pub unsafe extern "C" fn xi_trace_disable() {
    xi_trace::disable_tracing();
}

/// Enable tracing with the default configuration.  See
/// `xi_trace::enable_tracing`. Default is disabled.
#[no_mangle]
pub unsafe extern "C" fn xi_trace_enable() {
    xi_trace::enable_tracing();
}

#[no_mangle]
pub unsafe extern "C" fn xi_trace_is_enabled() -> bool {
    xi_trace::is_enabled()
}

/// C function for creating an instantaneous sample.  See `xi_trace::trace`.
/// If any of the arguments fail to parse (e.g. malformed UTF-8 or null pointer)
/// this function is a no-op.
///
/// # Performance
/// This is heavier-weight than invoking `xi_trace::trace` directly due to the
/// need to copy all values passed in by the caller into Rust objects.
///
/// # Arguments
///
/// `c_name` - A null-terminated UTF-8 string.
/// `c_categories` - A null-terminated array of null-terminated UTF-8 strings.
///
/// # Examples
///
/// ```text
/// xi_trace("something", (const char *[]){"ffi", "rpc"});
/// ```
#[no_mangle]
pub extern "C" fn xi_trace(c_name: *const c_char,
                           c_categories: *mut *const c_char)
{
    if !xi_trace::is_enabled() {
        return;
    }

    let name = c_from_str(c_name);
    let categories = c_from_categories(c_categories);

    if name.is_err() || categories.is_err() {
        if name.is_err() {
            eprintln!("Couldn't convert name: {}", name.unwrap_err());
        }

        if categories.is_err() {
            eprintln!("Couldn't convert categories: {}",
                      categories.unwrap_err());
        }
        return;
    }

    xi_trace::trace(name.unwrap(), categories.unwrap());
}

/// Creates a sample for a block of code.  The returned opaque value should be
/// passed to trace_block_end when the section of code to be measured completes.
/// Failure to call trace_block_end will result in a memory leak (maybe even if
/// tracing is disabled).  The returned value is opaque.
///
/// # Performance
///
/// See `trace`
///
/// # Arguments
/// See `trace`
///
/// # Examples
/// ```text
/// extern void* xi_trace_block_begin(const char *name, const char *categories[]);
/// extern void xi_trace_block_end(void* trace_block);
///
/// void *trace_block = xi_trace_block_begin("something", (const char *[]){"ffi", "rpc"});
/// xi_trace_block_end(trace_block);
/// ```
#[no_mangle]
pub extern "C" fn xi_trace_block_begin(c_name: *const c_char,
                                    c_categories: *mut *const c_char)
    -> *mut xi_trace::SampleGuard<'static> {
    if !xi_trace::is_enabled() {
        return std::ptr::null_mut();
    }

    let name = c_from_str(c_name);
    let categories = c_from_categories(c_categories);

    if name.is_err() || categories.is_err() {
        if name.is_err() {
            eprintln!("Couldn't convert name: {}", name.unwrap_err());
        }

        if categories.is_err() {
            eprintln!("Couldn't convert categories: {}",
                      categories.unwrap_err());
        }
        return std::ptr::null_mut();
    }

    let result = Box::new(xi_trace::trace_block(
            name.unwrap(), categories.unwrap()));
    return Box::into_raw(result);
}

/// Finalizes the block that was started via `trace_block_begin`.  See
/// `trace_block_begin` for more info.
#[no_mangle]
pub extern "C" fn xi_trace_block_end(
    trace_block: *mut xi_trace::SampleGuard<'static>) {
    if trace_block.is_null() {
        return;
    }

    unsafe {
        Box::from_raw(trace_block);
    }
}
