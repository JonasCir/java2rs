use crate::codegen::RustCodegen;
use crate::ir::*;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct MethodDeclarations {
    methods: Vec<MethodDeclaration>,
    functions: Vec<MethodDeclaration>,
}

impl MethodDeclarations {
    pub fn new() -> Self {
        MethodDeclarations {
            methods: Vec::new(),

            functions: Vec::new(),
        }
    }

    pub fn add_method(&mut self, method: MethodDeclaration) {
        if method.is_main() {
            self.functions.push(method)
        } else {
            self.methods.push(method)
        }
    }

    pub fn methods_to_rust(&self) -> proc_macro2::TokenStream {
        let methods: Vec<_> = self.methods.iter().map(RustCodegen::to_rust).collect();

        quote! {
            #(#methods);*
        }
    }

    pub fn functions_to_rust(&self) -> proc_macro2::TokenStream {
        let methods: Vec<_> = self.functions.iter().map(RustCodegen::to_rust).collect();

        quote! {
            #(#methods);*
        }
    }
}

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

    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    pub fn body_block(&self) -> &Block {
        &self.body_block
    }

    pub fn modifier(&self) -> &Modifier {
        &self.modifier
    }
}

impl RustCodegen for MethodDeclaration {
    fn to_rust(&self) -> TokenStream {
        let name = format_ident!("{}", &self.name);
        let body_block = self.body_block.to_rust();

        if self.is_main() {
            quote! {
                fn main()
                    #body_block
            }
        } else {
            assert!(self.parameters.is_empty());

            quote! {
                fn #name()
                    #body_block
            }
        }
    }
}

pub type Parameters = Vec<Parameter>;

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
