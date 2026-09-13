# 📘 **Basics of Generics**

A generic is a type parameter that allows code to work with different types while preserving type safety.

It acts as a placeholder for a future type, just as a function parameter acts as a placeholder for a future value.

For example, `Range<T>` is a generic type:

```text
Range<i32>  → `T` is replaced with `i32`
Range<char> → `T` is replaced with `char`
