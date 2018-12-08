#[derive(Debug, Clone, new)]
pub struct OrdinaryField {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub long: Option<String>,
    pub short: Option<String>,
}

#[derive(Debug, Clone, new)]
pub struct UnknownField {
    pub ident: syn::Ident,
    pub ty: syn::Type,
}

#[derive(Debug, Clone, new)]
pub struct PositionField {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub position: u64,
}

#[derive(Debug, Clone, new)]
pub enum Field {
    Ordinary(OrdinaryField),
    Unknown(UnknownField),
    Position(PositionField),
}

#[derive(Debug, Clone, new)]
pub struct StructData {
    pub ident: syn::Ident,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone)]
pub enum Data {
    Struct(StructData),
}

#[derive(Debug, Clone)]
enum Attribute {
    Long(String),
    Short(String),
    UnknownArgs,
    Position(u64),
}

pub fn parse_data(input: &syn::DeriveInput) -> Data {
    match input.data {
        syn::Data::Struct(ref struct_data) => Data::Struct(parse_data_from_struct(&input.ident, struct_data)),
        _ => panic!("Can only derive jockey::Arguments from struct."),
    }
}

fn parse_data_from_struct(ident: &syn::Ident, data: &syn::DataStruct) -> StructData {
    let fields = match data.fields {
        syn::Fields::Named(ref fields) => &fields.named,
        _ => panic!("Can only derive jockey::Arguments from struct with named fields"),
    };

    let field_data = fields.iter().map(|field| {
        let ident = field.ident.clone().unwrap();
        let ty = field.ty.clone();

        let mut is_unknown_args = false;
        let mut is_positional = false;
        let mut long_option = None;
        let mut short_option = None;
        let mut position = None;

        for attr in parse_attributes(&field.attrs) {
            match attr {
                Attribute::Long(val) => long_option = Some(String::from("--") + &val),
                Attribute::Short(val) => short_option = Some(String::from("-") + &val),
                Attribute::UnknownArgs => { is_unknown_args = true; },
                Attribute::Position(pos) => { is_positional = true; position = Some(pos); },
            }
        }

        if !is_unknown_args && !is_positional {
            if long_option.is_none() {
                long_option = Some(String::from("--") + &ident.to_string().replace("_", "-"));
            }
            Field::Ordinary(OrdinaryField::new(ident, ty, long_option, short_option))
        }
        else if is_unknown_args && !is_positional {
            Field::Unknown(UnknownField::new(ident, ty))
        }
        else if !is_unknown_args && is_positional {
            Field::Position(PositionField::new(ident, ty, position.unwrap()))
        }
        else {
            panic!();
        }
    }).collect();

    StructData::new(ident.clone(), field_data)
}

fn parse_attributes(attrs: &Vec<syn::Attribute>) -> Vec<Attribute> {
    attrs.iter().flat_map(|attr| {
        match parse_attribute(attr) {
            Some(list) => list.iter().map(|attr| match attr {
                (key, Some(val), None) => match key.as_ref() {
                    "long_option" => Attribute::Long(val.to_string()),
                    "short_option" => Attribute::Short(val.to_string()),
                    _ => panic!("Unknown attribute: {}", key),
                },
                (key, None, Some(val)) => match key.as_ref() {
                    "position" => Attribute::Position(*val),
                    _ => panic!("Unknown attribute: {}", key),
                }
                (key, None, None) => match key.as_ref() {
                    "unknown_args" => Attribute::UnknownArgs,
                    _ => panic!("Unknown attribute: {}", key),
                },
                _ => panic!(),
            }).collect(),
            None => vec![],
        }

    }).collect()
}

pub fn parse_attribute(attr: &syn::Attribute) -> Option<Vec<(String, Option<String>, Option<u64>)>> {
    match attr.parse_meta().unwrap() {
        syn::Meta::Word(ref ident) if ident.to_string() == "jockey" => {
            panic!("Bad use of jockey attribute (expected List)");
        }
        syn::Meta::NameValue(ref name_value) if name_value.ident == "jockey" => {
            panic!("Bad use of jockey attribute (expected List)");
        },
        syn::Meta::List(ref list) if list.ident == "jockey" => {
            Some(list.nested.iter().map(|nested| {
                match nested {
                    syn::NestedMeta::Meta(ref meta) => match meta {
                        syn::Meta::NameValue(ref name_value) => match name_value.lit {
                            syn::Lit::Str(ref value) => (name_value.ident.to_string(), Some(value.value()), None),
                            syn::Lit::Int(ref value) => (name_value.ident.to_string(), None, Some(value.value())),
                            _ => panic!("Bad use of jockey attribute (expected string literal"),
                        },
                        syn::Meta::Word(ref ident) => (ident.to_string(), None, None),
                        _ => panic!("Bad use of jockey attribute (expected Word or NameValue)"),
                    },
                    _ => panic!("Bad use of jockey attribute (expected Meta)"),
                }
            }).collect())
        },
        _ => None,
    }
}
