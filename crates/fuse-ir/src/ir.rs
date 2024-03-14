use fuse_ast::Identifier;

pub enum PrimitiveType {
    Number,
    String,
    Boolean,
}

impl PrimitiveType {
    /// All of the primitive types
    pub const ALL: [Self; 3] = [Self::Number, Self::Boolean, Self::String];

    pub fn from_identifier(ident: &Identifier) -> Option<Self> {
        match ident.name.as_str() {
            "number" => Some(Self::Number),
            "string" => Some(Self::String),
            "boolean" => Some(Self::Boolean),
            _ => None,
        }
    }
}
