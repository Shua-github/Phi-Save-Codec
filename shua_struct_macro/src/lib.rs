use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, parse_macro_input};

#[proc_macro_attribute]
pub fn binary_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let fields_named = match &input.data {
        Data::Struct(data) => {
            if let Fields::Named(fields) = &data.fields {
                fields.named.clone()
            } else {
                panic!("binary_struct only supports structs with named fields");
            }
        }
        _ => panic!("binary_struct only works on structs"),
    };

    let mut parse_stmts = Vec::new();
    let mut build_stmts = Vec::new();
    let mut field_names = Vec::new();

    for field in fields_named.iter() {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let field_name_str = field_name.to_string();

        let mut opt_func: Option<Ident> = None;
        for attr in &field.attrs {
            if attr.path().is_ident("binary_field") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("func") {
                        let value = meta.value()?;
                        let func_ident: Ident = value.parse()?;
                        opt_func = Some(func_ident);
                        Ok(())
                    } else {
                        Err(meta.error("expected `func`"))
                    }
                });
            }
        }

        field_names.push(field_name);

        let ctx_insert = quote! {
            ctx.insert(stringify!(#field_name).to_string(), Box::new(#field_name.clone()));
        };

        let pass_get_len = if let Some(func) = opt_func {
            quote! { Some(#func) }
        } else {
            quote! { outer_get_len }
        };

        parse_stmts.push(quote! {
            let #field_name = {
                let (val, l) = <#field_type as BinaryField>::parse(
                    &bits[offset..],
                    ctx,
                    Some(#field_name_str),
                    #pass_get_len
                )?;
                offset += l;
                val
            };
            #ctx_insert
        });

        build_stmts.push(quote! {
            bv.extend(self.#field_name.build());
        });
    }

    if let Data::Struct(ref mut data) = input.data {
        if let Fields::Named(ref mut fields) = data.fields {
            for field in fields.named.iter_mut() {
                field
                    .attrs
                    .retain(|attr| !attr.path().is_ident("binary_field"));
            }
        }
    }

    let expanded = quote! {
        #input

        impl BinaryField for #struct_name {
            fn parse(
                bits: &BitSlice<u8, Lsb0>,
                ctx: &mut Ctx,
                _name: Option<&str>,
                outer_get_len: Option<GetLen>,
            ) -> Result<(Self, usize), String> {
                let mut offset = 0;

                #(#parse_stmts)*

                Ok((Self {
                    #(#field_names),*
                }, offset))
            }

            fn build(&self) -> BitVec<u8, Lsb0> {
                let mut bv = BitVec::new();
                #(#build_stmts)*
                bv
            }
        }
    };

    TokenStream::from(expanded)
}
