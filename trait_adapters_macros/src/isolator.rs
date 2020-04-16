/*use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream, Result as ParseResult}, parse_macro_input, Token};
use quote::{format_ident, quote};

pub(crate) fn trait_isolator_impl(itrait: TokenStream) -> TokenStream {
    let input = parse_macro_input!(itrait as TraitIsolatorInput);
    let (tsrc, tdst) = (input.src_name(), input.dst_name());
    quote! {
        #[doc = "Only implements the `#tsrc` trait despite any other trait being implemented on `T`."]
        #[repr(transparent)]
        pub struct #tdst<T: #trrc> (pub T);
        impl<T: #tsrc> From<T> for #tdst<T> {
            #[inline(always)]
            fn from(op: T) -> Self {
                Self(op)
            }
        }
        impl<T: #tsrc> #tsrc for #tdst<T> {
            #[inline(always)]
            fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl<T: #tsrc> #tdst<T> {
            /// Consumes the adapter and returns the wrapped value.
            #[inline(always)]
            pub fn into_inner(self) -> T {
                self.0
            }
            /// Immutably borrows the inner value.
            #[inline(always)]
            pub fn inner(&self) -> &T {
                &self.0
            }
            /// Mutably borrows the inner value.
            #[inline(always)]
            pub fn inner_mut(&mut self) -> &mut T {
                &mut self.0
            }
        }
    }
}

struct TraitIsolatorInput {
    src_name: syn::Ident,
    dest_name: syn::Ident
}
impl TraitIsolatorInput {
    fn src_name(&self) -> &syn::Ident {
        &self.src_name
    }
    fn dest_name(&self) -> &syn::Ident {
        &self.dest_name
    }
}
impl Parse for TraitIsolatorInput {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        // Try to parse the source name.
        let src_name = syn::Ident::parse(input)?;
        let mut dest_name = format_ident!("Only{}", src_name);
        // If there's an as, search for the destination name:
        if let Ok(_) = <Token![as]>::parse(input) {
            dest_name = syn::Ident::parse(input)?;
        }
        Ok(Self {src_name, dest_name})
    }
}*/