use crate::codegen::RustCodegen;
use crate::ir::block::Block;
use crate::ir::modifier::Modifier;
use crate::ir::modifier::Visibility;
use crate::ir::r#type::{ArrayType, Dimensions, ScalarType, Type};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct MethodDeclaration {
    name: String,
    modifier: Modifier,
    parameters: Vec<Parameter>,
    return_type: Type,
    body_block: Block,
}

impl MethodDeclaration {
    pub fn new(
        name: String,
        modifier: Modifier,
        parameters: Vec<Parameter>,
        return_type: Type,
        body: Block,
    ) -> Self {
        MethodDeclaration {
            name,
            modifier,
            parameters,
            return_type,
            body_block: body,
        }
    }
    pub fn is_main(&self) -> bool {
        *self.modifier.visibility() == Visibility::Public
            && self.modifier.static_access()
            && self.return_type == Type::Scalar(ScalarType::Void)
            && self.name == "main"
            && self.parameters.len() == 1
            && *self.parameters.get(0).unwrap()
                == Parameter {
                    name: "args".to_string(),
                    ty: Type::Array(ArrayType::new(ScalarType::String, Dimensions::empty())),
                }
    }
}

impl RustCodegen for MethodDeclaration {
    fn to_rust(&self) -> TokenStream {
        assert!(self.modifier.static_access());
        assert_eq!(self.parameters.len(), 1);
        assert!(self.is_main());
        assert_eq!(self.parameters.len(), 1);
        let name = format_ident!("{}", &self.name);
        //let parameters = self.parameters.get(0).unwrap().to_rust();
        let body_block = self.body_block.to_rust();
        if self.is_main() {
            return quote! {
                fn #name()
                    #body_block
            };
        }
        todo!()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Parameter {
    name: String,
    ty: Type,
}

impl Parameter {
    pub fn new(name: String, ty: Type) -> Self {
        Parameter { name, ty }
    }
}

impl RustCodegen for Parameter {
    fn to_rust(&self) -> TokenStream {
        todo!()
    }
}

pub struct MethodInvocation {
    field_access: String,
    method_name: String,
    arguments: Arguments,
}

impl MethodInvocation {
    pub fn new(field_access: String, method_name: String, arguments: Arguments) -> Self {
        MethodInvocation {
            field_access,
            method_name,
            arguments,
        }
    }

    pub fn is_println(&self) -> bool {
        self.field_access == "System.out"
            && self.method_name == "println"
            && self.arguments.len() == 1
    }
}

impl RustCodegen for MethodInvocation {
    fn to_rust(&self) -> TokenStream {
        assert!(self.is_println());
        //  if self.is_println() {
        assert_eq!(self.arguments.len(), 1);

        let arg: syn::LitStr = syn::parse_str(self.arguments.get(0).unwrap())
            .expect("Unable to parse println1 argument");
        {
            quote! {
                println!(#arg)
            }
        }

        /*} else {
            self.field_access.to_owned()
                + "."
                + &self.method_name
                + "("
                + &self.arguments.join(",")
                + ")"
        }*/
    }
}

pub type Arguments = Vec<String>;
