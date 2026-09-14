use proc_macro2::Span;
use syn::{
    Path,
    parse_quote,
};
use quote::{
    format_ident,
};
use proc_macro_crate::{
    Error as FindCrateError,
    FoundCrate,
    crate_name,
};

pub fn get_isit_crate() -> Result<Path, FindCrateError> {
    let name = crate_name("isit")?;
    Ok(match name {
        FoundCrate::Itself => parse_quote!( crate ),
        FoundCrate::Name(name) => {
            let ident = format_ident!("{name}");
            parse_quote!( ::#ident )
        },
    })
}
