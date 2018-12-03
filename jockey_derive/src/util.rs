pub enum SupportedType {
    Unsupported,
    MandatoryString,
    OptionalString,
    Flag,
}

pub fn supported_type_to_string(ty: &syn::Type) -> SupportedType
{
    match ty {
        syn::Type::Path(ref path) => match path.path.segments.iter().next() {
            Some(first_seg) => match first_seg.ident.to_string().as_ref() {
                "String" => SupportedType::MandatoryString,
                "bool" => SupportedType::Flag,
                "Option" => match first_seg.arguments {
                    syn::PathArguments::AngleBracketed(ref ab) => match ab.args.iter().next() {
                        Some(ref arg) => match arg {
                            syn::GenericArgument::Type(ref ty) => match supported_type_to_string(ty) {
                                SupportedType::MandatoryString => SupportedType::OptionalString,
                                _ => SupportedType::Unsupported,
                            },
                            _ => SupportedType::Unsupported,
                        },
                        _ => SupportedType::Unsupported,
                    },
                    _ => SupportedType::Unsupported,
                }
                _ => SupportedType::Unsupported,
            }
            _ => SupportedType::Unsupported,
        }
        _ => SupportedType::Unsupported,
    }
}


