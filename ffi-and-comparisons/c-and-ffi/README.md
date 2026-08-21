# C Interop & FFI Barriers (C & Rust) 🔌

[← Back to Language Comparisons Index](../README.md)

C remains the lingua franca of systems engineering and operating systems interfaces. Rust provides first-class support for interoperating with C binaries and headers through Foreign Function Interfaces (FFI). However, crossing the FFI boundary requires stepping outside safe Rust guarantees.

---

## 1. Memory Layout & ABI Alignment

By default, Rust struct field ordering is unspecified for compiler optimization (field reordering to minimize padding). To interface with C, layout guarantees must be declared explicitly:

```rust
// Rust equivalent of C struct definition
#[repr(C)]
pub struct PacketHeader {
    pub magic: u16,
    pub payload_len: u32,
    pub flags: u8,
}
```

* **`#[repr(C)]`**: Guarantees identical memory layout, field ordering, and alignment as the host platform's C compiler.
* **`#[repr(packed)]`**: Removes padding bytes between fields (used for network protocol headers). *Warning*: Accessing unaligned fields can cause UB or hardware faults on strict alignment architectures.

---

## 2. Foreign Function Interface (`extern "C"`) & Symbol Mangling

### Calling C Functions from Rust
```rust
use std::ffi::c_int;

extern "C" {
    // Import function declaration from C static/dynamic library
    pub fn abs(input: c_int) -> c_int;
}

pub fn safe_abs(val: i32) -> i32 {
    // FFI calls are inherently unsafe
    unsafe { abs(val) }
}
```

### Exporting Rust Functions to C
```rust
// Prevent Rust compiler name mangling so C linker can resolve symbol name
#[no_mangle]
pub extern "C" fn rust_add_numbers(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 3. Strings & Pointer Safety Across FFI

C strings are null-terminated byte arrays (`char*`), whereas Rust strings (`String`, `&str`) store length explicitly and require valid UTF-8 encoding.

### Passing Rust Strings to C (`CString`)
```rust
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn c_log_message(msg: *const c_char);
}

pub fn log_to_c(message: &str) {
    // Converts Rust str into null-terminated CString
    let c_str = CString::new(message).expect("CString conversion failed");
    unsafe {
        c_log_message(c_str.as_ptr());
    } // c_str dropped here after call finishes
}
```

### Receiving C Strings in Rust (`CStr`)
```rust
use std::ffi::CStr;
use std::os::raw::c_char;

pub unsafe fn process_c_string(c_ptr: *const c_char) -> Option<String> {
    if c_ptr.is_null() {
        return None;
    }
    // Borrow null-terminated bytes and convert to lossy UTF-8 String
    let c_str = unsafe { CStr::from_ptr(c_ptr) };
    Some(c_str.to_string_lossy().into_owned())
}
```

---

## 4. Allocator Boundaries & Ownership Transfer

> [!CAUTION]
> **Allocator Mismatch Rule**: Memory allocated by Rust's allocator must be freed by Rust. Memory allocated by C's `malloc` must be freed by `free`. Mixing allocators across FFI leads to heap corruption.

### Transferring Ownership to C
```rust
#[no_mangle]
pub extern "C" fn create_buffer(size: usize) -> *mut u8 {
    let mut buf = vec![0u8; size];
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf); // Prevent Rust from deallocating buffer on scope exit
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn free_buffer(ptr: *mut u8, size: usize) {
    if !ptr.is_null() {
        // Reconstruct Vec to let Rust deallocate memory cleanly
        let _ = Vec::from_raw_parts(ptr, size, size);
    }
}
```

---

## 5. Automated Binding Tooling

* **`bindgen`**: Reads C header files (`.h`) and automatically generates safe and unsafe Rust bindings.
* **`cbindgen`**: Reads Rust crates and automatically generates standard C/C++ header files (`.h` / `.hpp`) for C consumers.
