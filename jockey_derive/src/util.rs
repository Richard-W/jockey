pub enum Type {
    MandatoryString,
    OptionalString,
    Flag,
}

impl From<syn::Type> for Type {
    fn from(ty: syn::Type) -> Self {
        match ty {
            syn::Type::Path(ref path) => match path.path.segments.iter().next() {
                Some(first_seg) => match first_seg.ident.to_string().as_ref() {
                    "String" => Type::MandatoryString,
                    "bool" => Type::Flag,
                    "Option" => match first_seg.arguments {
                        syn::PathArguments::AngleBracketed(ref ab) => match ab.args.iter().next() {
                            Some(ref arg) => match arg {
                                syn::GenericArgument::Type(ref ty) => match ty.clone().into() {
                                    Type::MandatoryString => Type::OptionalString,
                                    _ => panic!("Unsupported type"),
                                },
                                _ => panic!("Unsupported type"),
                            },
                            _ => panic!("Unsupported type"),
                        },
                        _ => panic!("Unsupported type"),
                    }
                    _ => panic!("Unsupported type"),
                }
                _ => panic!("Unsupported type"),
            }
            _ => panic!("Unsupported type"),
        }
    }
}

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
    pub ty: Type,
    pub long_option: String,
}

impl StructField {
    pub fn new(ident: syn::Ident, ty: Type, long_option: String) -> Self {
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
        let ty: Type = syn_struct_field.ty.clone().into();
        let long_option = "--".to_string() + &ident.to_string();

        struct_fields.push(StructField::new(ident, ty, long_option));
    }

    Struct::new(struct_ident, struct_fields)
}
