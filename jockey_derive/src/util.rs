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
}

impl StructField {
    pub fn new(ident: syn::Ident, ty: syn::Type, long_option: String) -> Self {
        StructField {
            ident: ident,
            ty: ty,
            long_option: long_option,
        }
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
        let long_option = "--".to_string() + &ident.to_string();

        struct_fields.push(StructField::new(ident, ty, long_option));
    }

    Struct::new(struct_ident, struct_fields)
}
