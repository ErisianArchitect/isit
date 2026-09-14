use syn::{
    Expr, ExprCall, LitStr, Token, parse::Parse
};
use quote::{
    quote,
    ToTokens,
};

pub enum ConstAssertions {
    Single(Box<Expr>),
    All(Vec<Self>),
    Any(Vec<Self>),
    Not(Vec<Self>),
}

impl ConstAssertions {
    fn from_expr(expr: Expr) -> Self {
        match expr {
            Expr::Call(ref call) => {
                if let Expr::Path(expr_path) = &*call.func {
                    if expr_path.path.is_ident("any") {
                        let exprs = call.args.iter()
                            .cloned()
                            .map(ConstAssertions::from_expr)
                            .collect::<Vec<_>>();
                        return Self::Any(exprs);
                    } else if expr_path.path.is_ident("all") {
                        let exprs = call.args.iter()
                            .cloned()
                            .map(ConstAssertions::from_expr)
                            .collect::<Vec<_>>();
                        return Self::All(exprs);
                    } else if expr_path.path.is_ident("none") || expr_path.path.is_ident("not") {
                        let exprs = call.args.iter()
                            .cloned()
                            .map(ConstAssertions::from_expr)
                            .collect::<Vec<_>>();
                        return Self::Not(exprs);
                    }
                }
                Self::Single(Box::new(expr))
            },
            expr => {
                Self::Single(Box::new(expr))
            }
        }
    }
}

impl Parse for ConstAssertions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse::<Expr>()?;
        Ok(Self::from_expr(expr))
    }
}

impl ToTokens for ConstAssertions {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        /*
            const {
                let result = 'assert_block: {
                    // all(any, none)
                    if !(
                        (
                            (cond1)
                            || (cond2)
                            || (cond3)
                            || (cond4)
                        )
                        && (
                            !(cond5)
                            && !(cond6)
                            && !(cond7)
                        )
                    )
                };
                if !result {
                    panic!(message);
                }
            }
        */
        tokens.extend(match self {
            ConstAssertions::Single(expr) => quote!( (#expr) ),
            ConstAssertions::All(items) => quote!( (#(#items)&&*) ),
            ConstAssertions::Any(items) => quote!( (#(#items)||*) ),
            ConstAssertions::Not(items) => quote!( (#(!#items)&&*) ),
        })
    }
}

pub struct ConstAssertInput {
    pub assertions: ConstAssertions,
    pub message: Option<LitStr>,
}

impl Parse for ConstAssertInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = input.parse::<ConstAssertions>()?;
        let message = if input.peek(Token![,]) {
            _ = input.parse::<Token![,]>()?;
            Some(input.parse::<LitStr>()?)
        } else {
            None
        };
        Ok(Self {
            assertions: expr,
            message,
        })
    }
}

struct ConstAssertionsMessage<'a>(&'a ConstAssertions);

impl<'a> ToTokens for ConstAssertionsMessage<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self.0 {
            ConstAssertions::Single(expr) => tokens.extend(quote!(#expr)),
            ConstAssertions::All(items) => {
                let all = items.iter()
                    .map(ConstAssertionsMessage);
                tokens.extend(quote!( all(#(#all),*)));
            },
            ConstAssertions::Any(items) => {
                let any = items.iter()
                    .map(ConstAssertionsMessage);
                tokens.extend(quote!( any(#(#any),*)));
            },
            ConstAssertions::Not(items) => {
                let items = items.iter()
                    .map(ConstAssertionsMessage);
                match items.len() {
                    1 => {
                        tokens.extend(quote!( not(#(#items),*) ));
                    }
                    _ => {
                        tokens.extend(quote!( none(#(#items),*) ));
                    }
                }
            },
        }
    }
}

impl ToTokens for ConstAssertInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let assertions = &self.assertions;
        let msg = self.message.as_ref();
        tokens.extend(if let Some(msg) = msg {
            quote! {
                const {
                    if !(#assertions) {
                        panic!(#msg);
                    }
                }
            }
        } else {
            let msg = ConstAssertionsMessage(assertions);
            quote! {
                const {
                    if !(#assertions) {
                        panic!(::core::concat!("Assertion failed: ", stringify!(#msg)));
                    }
                }
            }
        });
    }
}
