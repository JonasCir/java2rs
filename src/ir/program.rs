use crate::codegen::RustCodegen;
use crate::ir::class::ClassDeclaration;
use proc_macro2::TokenStream;

pub struct Program {
    class: ClassDeclaration,
}

impl Program {
    pub fn new(class: ClassDeclaration) -> Self {
        Program { class }
    }
}

impl RustCodegen for Program {
    fn to_rust(&self) -> TokenStream {
        self.class.to_rust()
    }
}
