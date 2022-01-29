use crate::codegen::RustCodegen;
use crate::ir::method::MethodDeclarations;
use crate::ir::modifier::Modifier;
use crate::ir::{Parameter, Statements};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct Class {
    name: String,
    modifier: Modifier,
    body: ClassBody,
}

impl Class {
    pub fn new(name: String, modifier: Modifier, body: ClassBody) -> Self {
        Class {
            name,
            modifier,
            body,
        }
    }
}

impl RustCodegen for Class {
    fn to_rust(&self) -> TokenStream {
        assert!(!self.modifier.static_access());
        let name = format_ident!("{}", &self.name);

        let struct_decl = quote! {
            struct #name {}
        };

        let methods = self.body.methods.methods_to_rust();
        let functions = self.body.methods.functions_to_rust();

        let constructor = if let Some(ref constructor) = self.body.constructor {
            constructor.to_rust()
        } else {
            quote! {}
        };

        let impl_block = if constructor.is_empty() && methods.is_empty() {
            quote! {}
        } else {
            quote! {
                impl #name {
                    #constructor
                    #methods
                }
            }
        };

        quote! {
            #struct_decl
            #impl_block
            #functions
        }
    }
}

pub struct ClassBody {
    constructor: Option<Constructor>,
    methods: MethodDeclarations,
}

impl ClassBody {
    pub fn new(constructor: Option<Constructor>, methods: MethodDeclarations) -> Self {
        ClassBody {
            constructor,
            methods,
        }
    }
}

pub struct Constructor {
    modifier: Modifier,
    parameters: Vec<Parameter>,
    body_block: ConstructorBody,
}

impl Constructor {
    pub fn new(
        modifier: Modifier,
        parameters: Vec<Parameter>,
        body_block: ConstructorBody,
    ) -> Self {
        Constructor {
            modifier,
            parameters,
            body_block,
        }
    }

    pub fn modifier(&self) -> &Modifier {
        &self.modifier
    }
    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }
    pub fn body_block(&self) -> &ConstructorBody {
        &self.body_block
    }
}

impl RustCodegen for Constructor {
    fn to_rust(&self) -> TokenStream {
        assert!(self.parameters().is_empty());

        let vis = self.modifier().visibility().to_rust();
        let body = self.body_block().to_rust();

        quote! {
            #vis fn new() -> Self
            #body
        }
    }
}

pub struct ConstructorBody {
    statements: Statements,
}

impl ConstructorBody {
    pub fn new(statements: Statements) -> Self {
        ConstructorBody { statements }
    }
}

impl RustCodegen for ConstructorBody {
    fn to_rust(&self) -> TokenStream {
        let statements: Vec<_> = self.statements.iter().map(RustCodegen::to_rust).collect();
        quote! {
            {
                #(#statements)*
                Self{}
            }
        }
    }
}
