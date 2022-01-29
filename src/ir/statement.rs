use crate::codegen::RustCodegen;

use crate::ir::MethodInvocation;
use proc_macro2::TokenStream;

pub enum ExpressionStatement {
    MethodInvocation(MethodInvocation),
}

impl RustCodegen for ExpressionStatement {
    fn to_rust(&self) -> TokenStream {
        match self {
            ExpressionStatement::MethodInvocation(method) => method.to_rust(),
        }
    }
}

pub type Statements = Vec<ExpressionStatement>;
