use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(PlyElement, attributes(ply))]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);

    match data {
        syn::Data::Struct(ds) => {
            let fields = ds.fields.iter().map(|f| {
                let name = &f.ident;
                // for a in &f.attrs {
                //     match &a.meta {
                //         syn::Meta::Path(p) => todo!("path {:?}", p.segments[0].ident),
                //         syn::Meta::List(l) => todo!("list {:?}", l.tokens),
                //         syn::Meta::NameValue(v) => todo!("meta"),
                //     }
                // }
                quote! {
                    #name: <_ as fast_ply::PlyProperty>::read::<B, _>(reader)?
                }
            });
            let output = quote! {
                impl fast_ply::PlyElement for #ident {
                    fn read<B: byteorder::ByteOrder, R: std::io::Read>(reader: &mut std::io::BufReader<R>) -> std::io::Result<Self>{
                        Ok(Self{
                            #(#fields,)*
                        })
                    }
                }
            };
            output.into()
        }
        _ => unimplemented!("not supported"),
    }
}
