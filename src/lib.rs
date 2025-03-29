use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, ItemStruct, Visibility, FieldsNamed
};


/// Makes both the struct itself and all its fields publicly accessible.
///
/// **To expose a struct and its fields outside the current module:**
/// 1. Annotate the struct definition with `pub`
/// 2. Explicitly mark each field with `pub` modifier
///
/// **This allows external code to:**
/// - Construct instances directly using struct literal syntax
/// - Read/modify individual fields without accessor methods
///
/// # Example
/// ```
/// pub struct Foo {
///     a: i32,
///     b: i32, 
/// }
///
/// // External code can:
/// let foo = Foo { a: 2, b: 10 };
/// assert_eq!(foo.a, 2);
/// assert_eq!(foo.b, 10);
/// ```
/// 
/// # Note
/// Struct visibility also depends on parent module's visibility[1,4](@ref). 
/// The containing module must be public to allow cross-module access.
///
/// For crate-internal visibility, consider `pub(crate)` instead[1,2](@ref).
#[proc_macro_attribute]
pub fn public_fields(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the original struct
    let mut input = parse_macro_input!(item as ItemStruct);
    
    // Make the struct itself public
    input.vis = Visibility::Public(Default::default());

    // Make all fields in the struct public
    if let syn::Fields::Named(FieldsNamed { 
        named: fields, .. 
    }) = &mut input.fields {
        for field in fields.iter_mut() {
            field.vis = Visibility::Public(Default::default());
        }
    }

    // Get the new struct
    let expanded = quote! { #input };
    TokenStream::from(expanded)
}
