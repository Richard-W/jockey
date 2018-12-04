pub struct Struct {
    pub ident: syn::Ident,
    pub fields: Vec<StructField>,
}

impl Struct {
    pub fn new(ident: syn::Ident, fields: Vec<StructField>) -> Self {
        Struct {
            ident: ident,
            fields: fields,
        }
    }
}

#[derive(Clone)]
pub struct StructField {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub long_option: Option<String>,
    pub short_option: Option<String>,
    pub unknown_args: bool,
}

impl StructField {
    pub fn new(ident: syn::Ident, ty: syn::Type) -> Self {
        StructField {
            ident: ident,
            ty: ty,
            long_option: None,
            short_option: None,
            unknown_args: false,
        }
    }
}

pub fn apply_field_attrs(target: &mut StructField, attr: &syn::Attribute) {
    match attr.parse_meta().unwrap() {
        syn::Meta::Word(ident) => {
            if ident == "jockey" {
                panic!("Bad use of jockey attribute (expected List)");
            }
        },
        syn::Meta::NameValue(name_value) => {
            if name_value.ident == "jockey" {
                panic!("Bad use of jockey attribute (expected List)");
            }
        },
        syn::Meta::List(list) => if list.ident == "jockey" {
            for nested in list.nested.iter() {
                match nested {
                    syn::NestedMeta::Meta(nested_meta) => match nested_meta {
                        syn::Meta::NameValue(name_value) => match &name_value.lit {
                            syn::Lit::Str(ref value) => handle_field_attr(
                                target,
                                &name_value.ident.to_string(),
                                Some(value.value())
                            ),
                            _ => panic!("Bad use of jockey attribute (expected string literal"),
                        },
                        syn::Meta::Word(ident) => handle_field_attr(
                            target,
                            &ident.to_string(),
                            None
                        ),
                        _ => panic!("Bad use of jockey attribute (expected NameValue)"),
                    },
                    _ => panic!("Bad use of jockey attribute (expected Meta)"),
                }
            }
        },
    }
}

pub fn handle_field_attr(target: &mut StructField, key: &str, value: Option<String>) {
    match value {
        Some(value) => match key {
            "long_option" => target.long_option = Some(value),
            "short_option" => target.short_option = Some(value),
            _ => panic!("Unknown attribute key: {}", key),
        },
        None => match key {
            "unknown_args" => target.unknown_args = true,
            _ => panic!("Unknown jockey attribute: {}", key),
        }
    }
}

pub fn derive_input_to_struct_def(input: &syn::DeriveInput) -> Struct {
    let syn_fields = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => panic!("Can only derive Arguments from struct with named fields"),
        }
        _ => panic!("Can only derive Arguments from struct"),
    };

    let mut fields: Vec<StructField> = Vec::new();
    for syn_field in syn_fields {
        let mut field = StructField::new(syn_field.ident.clone().unwrap(), syn_field.ty.clone());

        for attr in syn_field.attrs.clone() {
            apply_field_attrs(&mut field, &attr);
        }

        if field.unknown_args {
            if field.long_option.is_some() || field.short_option.is_some() {
                panic!("unknown_args field cannot have option string assigned");
            }
        }
        else {
            if field.long_option.is_none() {
                // Long option name is not set. Make up one based on the field name.
                field.long_option = Some(field.ident.to_string().replace("_", "-"));
            }
        }

        fields.push(field);
    }

    Struct::new(input.ident.clone(), fields)
}
