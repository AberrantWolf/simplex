use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

#[proc_macro_derive(XmlStruct)]
pub fn xml_struct_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_xml_data(&ast)
}

fn impl_xml_data(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    match &data {
        syn::Data::Struct(struct_data) => {
            let fields = &struct_data.fields;
            match &fields {
                syn::Fields::Named(named_fields) => {
                    let field_iter = named_fields.named.iter();
                    let field_idents: Vec<&Ident> =
                        field_iter.map(|f| f.ident.as_ref().unwrap()).collect();
                    let gen = quote! {
                        impl XmlStruct for #name {
                            fn from_xml(xml_string: String) -> Result<Self, XmlStructError> {
                                let mut reader = Reader::from_str(xml_string.as_str());
                                let mut result = Self::default();

                                loop {
                                    match reader.read_event() {
                                        Err(e) => return Err(XmlStructError::SomethingBad {source: e}),
                                        Ok(Event::Eof) => break,
                                        Ok(Event::Start(e)) => {
                                            if let b"user" = e.name().as_ref() {
                                                #(if let Some(attr) = e.try_get_attribute(stringify!(#field_idents))? {
                                                    result.#field_idents = attr.unescape_value()?.to_string().parse()?;
                                                })else *
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                Err(XmlStructError::Temp)
                            }
                        }
                    };
                    gen.into()
                }
                syn::Fields::Unnamed(_) => todo!(),
                syn::Fields::Unit => todo!(),
            }
        }
        syn::Data::Enum(_) => panic!("Enum not supported"),
        syn::Data::Union(_) => panic!("Union not supported"),
    }
}
