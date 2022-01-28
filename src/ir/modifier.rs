use crate::codegen::RustCodegen;
use proc_macro2::TokenStream;
pub struct Modifier {
    visibility: Visibility,
    static_access: bool,
}

impl Modifier {
    pub fn new(visibility: Visibility, static_access: StaticAccess) -> Self {
        Modifier {
            visibility,
            static_access: static_access.into(),
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
    Public,
}

impl RustCodegen for Visibility {
    fn to_rust(&self) -> TokenStream {
        /*match self {
            Visibility::Public => "pub".to_string(),
        }*/
        todo!()
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
