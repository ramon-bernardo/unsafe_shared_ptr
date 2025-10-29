**Shared** is a low-level smart pointer that provides manual reference-counted memory management for a single value. Inspired by Rc<T>, it allows allocating, accessing and freeing memory directly using unsafe primitives. The project demonstrates key unsafe operations, including raw pointer manipulation, manual reference counting and explicit dropping of values.

### Key features:
- Allocates memory on the heap manually using alloc and dealloc.
- Supports shared ownership via manual reference counting.
- Provides safe-style access through Deref and DerefMut.
- Supports any Rust type, including primitives, structs and collections.
- Ensures proper cleanup when the last reference is dropped.
- Includes tests for reading, writing, mutating, cloning and verifying memory is freed correctly.
