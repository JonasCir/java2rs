use crate::codegen::RustCodegen;
use crate::ir::class::Class;
use proc_macro2::TokenStream;

pub struct Program {
    class: Class,
}

impl Program {
    pub fn new(class: Class) -> Self {
        Program { class }
    }
}

impl RustCodegen for Program {
    fn to_rust(&self) -> TokenStream {
        self.class.to_rust()
    }
}
