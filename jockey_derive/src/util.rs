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
    pub fn new(ident: syn::Ident, ty: syn::Type, long_option: String, short_option: Option<String>) -> Self {
        StructField {
            ident: ident,
            ty: ty,
            long_option: long_option,
            short_option: short_option,
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
        "short_option" => target.short_option = Some(value),
        _ => panic!("Unknown jockey attribute: {}", key),
    }
}

pub fn derive_input_to_struct_def(input: &syn::DeriveInput) -> Struct {
    let struct_ident: syn::Ident = input.ident.clone();

    let syn_struct_fields = match input.data {
        syn::Data::Struct(ref data) => match data.fields {
            syn::Fields::Named(ref fields) => &fields.named,
            _ => panic!("Can only derive JockeyArguments from struct with named fields"),
        }
        _ => panic!("Can only derive JockeyArguments from struct"),
    };

    let mut struct_fields: Vec<StructField> = Vec::new();
    for syn_struct_field in syn_struct_fields {
        let ident = syn_struct_field.ident.clone().unwrap();
        let ty = syn_struct_field.ty.clone();
        let mut long_option = "--".to_string() + &ident.to_string();
        let mut short_option = None;

        let mut struct_field = StructField::new(ident, ty, long_option, short_option);

        for attr in syn_struct_field.attrs.clone() {
            apply_field_attrs(&mut struct_field, &attr);
        }

        struct_fields.push(struct_field);
    }

    Struct::new(struct_ident, struct_fields)
}
