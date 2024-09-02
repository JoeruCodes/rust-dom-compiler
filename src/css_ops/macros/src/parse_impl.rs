use proc_macro::TokenStream;
use syn::{
    parse::{Parse, ParseStream}, punctuated::Punctuated, token::Comma, Error, Ident, ItemEnum, ItemStruct, LitBool, LitStr, Result
};
use syn::Lit;
pub struct Args {
    pub sister: Vec<String>,
    pub inherit: bool,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut sister = Vec::new();
        let mut inherit = false;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let _: syn::Token![:] = input.parse()?;

            match ident.to_string().as_str() {
                "sister" => {
                    let content;
                    syn::bracketed!(content in input);
                    sister = Punctuated::<LitStr, Comma>::parse_terminated(&content)?
                        .into_iter()
                        .map(|lit| lit.value())
                        .collect();
                }
                "inherit" => {
                    let value: LitBool = input.parse()?;
                    inherit = value.value;
                }
                _ => return Err(Error::new(ident.span(), "unexpected field")),
            }

            // Parse optional comma
            if input.peek(Comma) {
                let _: Comma = input.parse()?;
            }
        }

        Ok(Args { sister, inherit })
    }
}

pub struct ArgsStruct {
    pub inherit: bool,
    pub initial_inherit_wrap: Vec<String>
}

impl Parse for ArgsStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut inherit = false;
        let mut initial_inherit_wrap = Vec::new();
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let _: syn::Token![:] = input.parse()?;

            match ident.to_string().as_str() {
                "inherit" => {
                    let value: LitBool = input.parse()?;
                    inherit = value.value;
                },
                "initial_inherit_wrap" => {
                    let content;
                    syn::bracketed!(content in input);
                    initial_inherit_wrap = Punctuated::<LitStr, Comma>::parse_terminated(&content)?
                        .into_iter()
                        .map(|lit| lit.value())
                        .collect();
                }
                _ => return Err(Error::new(ident.span(), "unexpected field")),
            }

            // Parse optional comma
            if input.peek(Comma) {
                let _: Comma = input.parse()?;
            }
        }

        Ok(ArgsStruct {inherit, initial_inherit_wrap })
    }
}
pub enum Items{
    Enum(ItemEnum),
    Struct(ItemStruct)
}

impl Items{
    pub fn from(item: TokenStream) -> Self{
        return match syn::parse::<ItemStruct>(item.clone()){
            Ok(item_struct) => Items::Struct(item_struct),
            Err(_) => {
                match syn::parse::<ItemEnum>(item.clone()){
                    Ok(item_enum) => Items::Enum(item_enum),
                    Err(_) => panic!()
                }
            }
        };
    }
}