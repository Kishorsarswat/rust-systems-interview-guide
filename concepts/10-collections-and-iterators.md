# Topic 10: Collections & Iterators

[← Back to Concepts Index](./README.md)

---

### Questions

1. **What's the difference between `Vec`, `VecDeque`, and `LinkedList`? When (if ever) would you use `LinkedList`?**
   * *Key aspects to address:* Contiguous array cache-locality (`Vec`), ring-buffer double-ended queue (`VecDeque`), non-contiguous node pointers (`LinkedList`). Cache misses making `Vec`/`VecDeque` almost universally superior to `LinkedList` on modern CPUs.

2. **Explain the Entry API on `HashMap`. What problem does it solve compared to manual `get`/`insert`?**
   * *Key aspects to address:* In-place lookup and insertion (`Occupied` vs `Vacant`), avoiding double hash lookups, clean in-place mutations (`or_insert`, `or_default`, `and_modify`).

3. **What's the difference between `iter()`, `iter_mut()`, and `into_iter()`?**
   * *Key aspects to address:* Borrowing immutable references (`&T`), borrowing mutable references (`&mut T`), taking ownership / consuming the collection (`T`).

4. **What does it mean that iterators are lazy? What triggers evaluation?**
   * *Key aspects to address:* Iterator adapters (`map`, `filter`, `take`) constructing iterator wrappers without performing computation until consumed by a driver method (`collect`, `for_each`, `fold`, `sum`).

5. **Walk through what `flat_map`, `fold`, `zip`, and `enumerate` do.**
   * *Key aspects to address:* Iterator transformation mechanics, flattening nested iterators, accumulator state aggregation, pairing elements across two streams, index tracking.

6. **What's the difference between `map` + `collect` and writing an explicit loop, in terms of performance?**
   * *Key aspects to address:* Internal iteration optimization, auto-vectorization (SIMD), bounds check elimination, `ExactSizeIterator` capacity allocation in `collect`.

7. **What is the difference between `copied()` and `cloned()`?**
   * *Key aspects to address:* Dereferencing `&T` for types implementing `Copy` (shallow byte copy) vs calling `.clone()` for types implementing `Clone` (potential heap allocation/deep copy).
