mod const_assert;

mod util;

use quote::{quote};
use proc_macro::TokenStream;

/// Perform assertions at compile time to prevent builds when they fail.
/// 
/// Can be invoked either with or without a message, just like panic (except
/// messages must be literals, not format strings).
/// 
/// When a message is not used, the message will be the expression itself.
/// 
/// ```rust, ignore
/// use isit_macros::const_assert;
/// 
/// // With Messages
/// const_assert!(true, "This will not panic.");
/// const_assert!(false, "This will panic.");
/// // Without Messages
/// const_assert!(true);
/// const_assert!(false);
/// ```
/// 
/// # Examples
/// ```rust, ignore
/// use isit_macros::const_assert;
/// 
/// fn main() {
///     // Will not panic.
///     const_assert!(true);
///     // Will panic
///     const_assert!(false);
///     // Any condition (will not panic)
///     const_assert!(
///         any(true, false)
///     );
///     // Any condition
///     // (will panic)
///     const_assert!(
///         any(false, false)
///     );
///     // All condition
///     // (will not panic)
///     const_assert!(
///         all(true, true)
///     );
///     // All condition
///     // (will panic)
///     const_assert!(
///         all(false, true)
///     );
///     // All condition
///     // (will panic)
///     const_assert!(
///         all(false, false)
///     );
///     // None condition
///     // (will not panic)
///     const_assert!(
///         none(false, false)
///     );
///     // None condition
///     // (will panic)
///     const_assert!(
///         none(false, false, true)
///     );
///     // Not condition (alias for None)
///     // (will not panic)
///     const_assert!(
///         not(false, false)
///     );
///     // Not condition (alias for None)
///     // (will panic)
///     const_assert!(
///         not(false, false, true)
///     );
///     // Nesting of conditions
///     const_assert!(
///         all(
///             any(true, false),
///             none(
///                 false,
///                 not(true)
///             )
///         )
///     );
/// }
/// ```
#[proc_macro]
pub fn const_assert(input: TokenStream) -> TokenStream {
    let const_assert_input = syn::parse_macro_input!(input as const_assert::ConstAssertInput);
    quote! { #const_assert_input }.into()
}
