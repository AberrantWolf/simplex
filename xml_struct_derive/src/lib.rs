use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, Field, Ident, LitByteStr};

#[proc_macro_derive(XmlStruct, attributes(xmlElement, xmlName))]
pub fn xml_struct_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_xml_data(ast)
}

enum SupportedAttributes {
    Nothing,
    XmlElement,
}

fn filter_supported_attribute_paths(attr: &&Attribute) -> SupportedAttributes {
    if attr.path.is_ident("xmlElement") {
        return SupportedAttributes::XmlElement;
    }
    SupportedAttributes::Nothing
}

fn filter_xml_name(field: &Field) -> String {
    for attr in &field.attrs {
        if attr.path.is_ident("xmlName") {
            if let Ok(meta) = attr.parse_meta() {
                match meta {
                    syn::Meta::Path(_) => unimplemented!(),
                    syn::Meta::List(_) => unimplemented!(),
                    syn::Meta::NameValue(name_value) => {
                        return match name_value.lit {
                            syn::Lit::Str(s) => s.value(),
                            syn::Lit::ByteStr(s) => String::from_utf8(s.value()).unwrap(),
                            syn::Lit::Byte(_) => unimplemented!(),
                            syn::Lit::Char(_) => unimplemented!(),
                            syn::Lit::Int(_) => unimplemented!(),
                            syn::Lit::Float(_) => unimplemented!(),
                            syn::Lit::Bool(_) => unimplemented!(),
                            syn::Lit::Verbatim(_) => unimplemented!(),
                        };
                    }
                }
            }
        }
    }

    format!("{}", &field.ident.as_ref().unwrap())
}

fn impl_xml_data(ast: syn::DeriveInput) -> TokenStream {
    let name = ast.ident;
    let data = ast.data;
    let attrs = ast.attrs;

    let mut container_elem_name_string: LitByteStr =
        LitByteStr::new(format!("{}", name).as_bytes(), Span::call_site());
    attrs.iter().for_each(|a| {
        let support = filter_supported_attribute_paths(&a);

        match support {
            SupportedAttributes::Nothing => return,
            SupportedAttributes::XmlElement => {
                if let Ok(meta) = a.parse_meta() {
                    match meta {
                        syn::Meta::Path(_) => unimplemented!(),
                        syn::Meta::List(_) => unimplemented!(),
                        syn::Meta::NameValue(name_value) => {
                            let str: String = match name_value.lit {
                                syn::Lit::Str(s) => s.value(),
                                syn::Lit::ByteStr(s) => String::from_utf8(s.value()).unwrap(),
                                syn::Lit::Byte(_) => unimplemented!(),
                                syn::Lit::Char(_) => unimplemented!(),
                                syn::Lit::Int(_) => unimplemented!(),
                                syn::Lit::Float(_) => unimplemented!(),
                                syn::Lit::Bool(_) => unimplemented!(),
                                syn::Lit::Verbatim(_) => unimplemented!(),
                            };
                            container_elem_name_string =
                                LitByteStr::new(str.as_bytes(), Span::call_site());
                        }
                    }
                }
            }
        }
    });

    match data {
        syn::Data::Struct(struct_data) => {
            let fields = &struct_data.fields;
            match &fields {
                syn::Fields::Named(named_fields) => {
                    let field_iter = named_fields.named.iter();
                    let (field_idents, field_names): (Vec<&Ident>, Vec<String>) = field_iter
                        .map(|f| (f.ident.as_ref().unwrap(), filter_xml_name(f)))
                        .unzip();

                    let gen = quote! {
                        impl XmlStruct for #name {
                            fn from_xml(xml_string: String) -> xml_struct::Result<Self> {
                                let mut reader = Reader::from_str(xml_string.as_str());
                                let mut result = Self::default();

                                loop {
                                    match reader.read_event() {
                                        Err(e) => return Err(XmlStructError::SomethingBad {source: e}),
                                        Ok(Event::Eof) => break,
                                        Ok(Event::Start(e)) => {
                                            if let #container_elem_name_string = e.name().as_ref() {
                                                #(if let Some(attr) = e.try_get_attribute(#field_names)? {
                                                    result.#field_idents = attr.unescape_value()?.to_string().parse()?;
                                                    println!("Found {}", stringify!(#field_idents));
                                                })*
                                            }
                                        }
                                        _ => (),
                                    }
                                }
                                Ok(result)
                            }
                        }
                    };
                    gen.into()
                }
                syn::Fields::Unnamed(_) => unimplemented!(),
                syn::Fields::Unit => unimplemented!(),
            }
        }
        syn::Data::Enum(_) => unimplemented!("Enum from XML not yet implemented"),
        syn::Data::Union(_) => unimplemented!("Union from XML not yet implemented"),
    }
}
