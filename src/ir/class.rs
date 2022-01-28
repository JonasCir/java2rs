use crate::codegen::RustCodegen;
use crate::ir::method::MethodDeclaration;
use crate::ir::modifier::Modifier;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct Class {
    name: String,
    modifier: Modifier,
    methods: Vec<MethodDeclaration>,
    static_methods: Vec<MethodDeclaration>,
}

impl Class {
    pub fn new(
        name: String,
        modifier: Modifier,
        methods: Vec<MethodDeclaration>,
        static_methods: Vec<MethodDeclaration>,
    ) -> Self {
        Class {
            name,
            modifier,
            methods,
            static_methods,
        }
    }
}

impl RustCodegen for Class {
    fn to_rust(&self) -> TokenStream {
        assert!(!self.modifier.static_access());
        let name = format_ident!("{}", &self.name);
        assert!(self.methods.is_empty());

        let struct_decl = quote! {
            struct #name {}
        };

        assert_eq!(self.methods.len(), 0);
        assert_eq!(self.static_methods.len(), 1);
        let method_decl = self.static_methods.get(0).unwrap();
        assert!(method_decl.is_main());

        let main = method_decl.to_rust();

        quote! {
            #struct_decl

            #main
        }
    }
}
