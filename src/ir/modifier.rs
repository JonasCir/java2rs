use crate::codegen::RustCodegen;
use proc_macro2::TokenStream;
use quote::quote;

pub struct Modifier {
    visibility: Visibility,
    static_access: bool,
}

impl Modifier {
    pub fn new(visibility: Visibility, static_access: StaticAccess) -> Self {
        Self {
            visibility,
            static_access: static_access.into(),
        }
    }
    pub fn empty() -> Self {
        Self {
            visibility: Visibility::Default,
            static_access: false,
        }
    }

    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }
    pub fn static_access(&self) -> bool {
        self.static_access
    }
}
impl RustCodegen for Modifier {
    fn to_rust(&self) -> TokenStream {
        self.visibility.to_rust()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Visibility {
    Default,
    Public,
}

impl RustCodegen for Visibility {
    fn to_rust(&self) -> TokenStream {
        match self {
            Visibility::Public => quote! {pub},
            _ => todo!("Not sure yet what default visibility is in Rust"),
        }
    }
}
#[derive(Debug, Eq, PartialEq)]
pub enum StaticAccess {
    Static,
    NotStatic,
}

impl From<StaticAccess> for bool {
    fn from(access: StaticAccess) -> Self {
        match access {
            StaticAccess::Static => true,
            StaticAccess::NotStatic => false,
        }
    }
}
