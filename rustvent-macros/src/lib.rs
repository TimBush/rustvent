use core::panic;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, format_ident, ToTokens};
use syn::{self, DeriveInput, Data, Type};

#[proc_macro_derive(Event)]
pub fn event_macro_derive(item: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(item);
    let DeriveInput { ident, data, .. } = syn::parse2(input).unwrap();
    let mut trait_func_stream = TokenStream2::default();
    let mut impl_func_stream = TokenStream2::default();

    let ftrait_name = format_ident!("Rustvent{}", &ident);

    let output = if let Data::Struct(x) = data {

        let fields = x.fields.iter().map(|f| {
            (&f.ident, &f.ty)
        });
  
        for (field, ty) in fields.clone().into_iter() {
            if !is_typeof_event(ty) { continue; }

            let fname = format_ident!("on_{}", field.clone().unwrap());

            trait_func_stream.extend::<TokenStream2>(
                quote! {
                    fn #fname(&mut self);
                }
            );
        }

        for (field, ty) in fields.clone().into_iter() {
            if !is_typeof_event(ty) { continue; }

            let name = field.clone().unwrap();
            let fname = format_ident!("on_{}", field.clone().unwrap());

            impl_func_stream.extend::<TokenStream2>(
                quote! {
                    fn #fname(&mut self) {
                        self.#name.notify();
                    }
                }
            );
        }

        let trait_def = quote! {
            pub trait #ftrait_name {
                #trait_func_stream
            }
        };

        let impl_def = quote! {
            impl #ftrait_name for #ident {
                #impl_func_stream
            }
        };

        quote! {
            #trait_def
            #impl_def
        }
    } else {
        panic!("Macro was not used on a Struct");
    };

    output.into()

}

fn is_typeof_event(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) if type_path.clone().into_token_stream().to_string() == "Event" => {
            true
        },
        _ => false
    }
}