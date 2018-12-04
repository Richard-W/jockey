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

pub struct StructField {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub long_option: String,
    pub short_option: Option<String>,
}

impl StructField {
    pub fn new(ident: syn::Ident, ty: syn::Type) -> Self {
        StructField {
            ident: ident,
            ty: ty,
            long_option: "".to_string(),
            short_option: None,
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
                            syn::Lit::Str(ref value) => handle_field_attr(target, &name_value.ident.to_string(), value.value()),
                            _ => panic!("Bad use of jockey attribute (expected string literal"),
                        },
                        _ => panic!("Bad use of jockey attribute (expected NameValue)"),
                    },
                    _ => panic!("Bad use of jockey attribute (expected Meta)"),
                }
            }
        },
    }
}

pub fn handle_field_attr(target: &mut StructField, key: &str, value: String) {
    match key {
        "long_option" => target.long_option = value,
        "short_option" => target.short_option = Some(value),
        _ => panic!("Unknown jockey attribute: {}", key),
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

        // If no long option was set add it
        if field.long_option.len() == 0 {
            field.long_option = field.ident.to_string();
        }

        fields.push(field);
    }

    Struct::new(input.ident.clone(), fields)
}
