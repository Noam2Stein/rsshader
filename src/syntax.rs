pub struct Module {

}

pub struct Type {
    pub ident: Ident,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attribute {
    pub ident: Ident,
    pub fields: Option<Vec<AttributeField>>,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttributeField {
    Literal(Literal),
    Attribute(Attribute),
}

pub struct Function {
    pub attributes: Vec<Attribute>,
    pub is_pub: bool,
    pub is_const: bool,
    pub ident: Ident,
    pub fields: Vec<FunctionField>,
    pub output: Option<Type>,
    pub block: Option<Block>,
}
pub struct FunctionField {
    pub ident: Ident,
    pub ty: Type,
}

pub struct Block {
    pub tts: TokenStream,
}