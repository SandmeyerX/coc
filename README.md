# public_fields
Makes both the struct itself and all its fields publicly accessible.

**To expose a struct and its fields outside the current module:**
1. Annotate the struct definition with `pub`
2. Explicitly mark each field with `pub` modifier

**This allows external code to:**
- Construct instances directly using struct literal syntax
- Read/modify individual fields without accessor methods

## Example
 ```rust
pub struct Foo {
    a: i32,
    b: i32, 
}

// External code can:
let foo = Foo { a: 2, b: 10 };
assert_eq!(foo.a, 2);
assert_eq!(foo.b, 10);
 ```

## Note
Struct visibility also depends on parent module's visibility[1,4](@ref). 
The containing module must be public to allow cross-module access.

For crate-internal visibility, consider `pub(crate)` instead[1,2](@ref).
