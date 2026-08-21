# C++ to Rust Systems Paradigms ⚙️

[← Back to Language Comparisons Index](../README.md)

For systems engineers transitioning from C++ to Rust, understanding the mapping between high-performance C++ constructs and safe Rust primitives is crucial for writing idiomatic code.

---

## 1. Smart Pointers & Memory Management

| C++ Abstraction | Rust Equivalent | Key Operational Differences |
|-----------------|-----------------|-----------------------------|
| `std::unique_ptr<T>` | `Box<T>` | `Box<T>` is non-null by default. Transfer of ownership invalidates source variable at compile time without leaving a hollowed-out pointer. |
| `std::shared_ptr<T>` | `Rc<T>` / `Arc<T>` | C++ `std::shared_ptr` uses atomic ref counting by default. Rust splits non-thread-safe (`Rc`) and thread-safe (`Arc`) to avoid atomic penalty when multi-threading is unneeded. |
| `std::weak_ptr<T>` | `Weak<T>` | Works similarly to break cyclic references. Accessing inner data requires `.upgrade() -> Option<Arc<T>>`. |
| `std::vector<T>` | `Vec<T>` | Dynamic resizable heap array. Continuous memory storage. |

---

## 2. RAII: Destruction & Resource Management

### C++ RAII Destructor
```cpp
class FileLogger {
    FILE* handle;
public:
    FileLogger(const char* path) { handle = fopen(path, "w"); }
    ~FileLogger() { if (handle) fclose(handle); } // RAII Destructor
};
```

### Rust `Drop` Trait Equivalent
```rust
pub struct FileLogger {
    handle: *mut std::ffi::c_void,
}

impl Drop for FileLogger {
    fn drop(&mut self) {
        // Drop trait executes automatically when variable leaves scope
        println!("Cleaning up FileLogger resource");
    }
}
```

---

## 3. Move Semantics: C++ `std::move` vs Rust Moves

* **C++ Move (`std::move`)**: Casts lvalue to rvalue reference (`T&&`). Requires move constructors/assignment operators. The moved-from object remains valid (in an "unspecified state") and its destructor still runs.
* **Rust Move (Default)**: All assignments and parameter passes are Moves by default (unless `Copy` is implemented). A move is a byte-for-byte `memcpy`. The compiler marks the source variable unusable, guaranteeing no double-free or destructor execution on the source.

---

## 4. Templates vs Rust Generics

### C++ Template (Duck Typing at Compile Time)
```cpp
template <typename T>
T add(T a, T b) {
    return a + b; // Compiled lazily when instantiated; fails if '+' is missing
}
```

### Rust Generic with Trait Bounds (Parametric Polymorphism)
```rust
use std::ops::Add;

// Checked at definition time against Add trait contract
pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
```

---

## 5. Locking Abstractions: Data-centric vs Scope-centric

### C++ `std::mutex` (Unbound Lock)
```cpp
std::mutex mtx;
int data = 0;

void increment() {
    std::lock_guard<std::mutex> lock(mtx); // Mutex protects block of code, not data directly
    data++;
}
```

### Rust `Mutex<T>` (Data Encapsulation)
```rust
use std::sync::Mutex;

struct SharedData {
    counter: Mutex<i32>, // Data lives INSIDE the Mutex
}

fn increment(shared: &SharedData) {
    let mut guard = shared.counter.lock().unwrap(); // Lock guard derefs directly to data
    *guard += 1;
} // Lock released automatically when guard is dropped
```
