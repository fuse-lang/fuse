use crate::ast::*;

#[derive(Debug, Clone, Copy)]
pub enum AstNode<'a> {
    // Primary nodes
    Chunk(&'a Chunk),
    Block(&'a Block),

    // Statement related
    EmptyStatement(&'a EmptyStatement),
    ImplStatement(&'a ImplStatement),
    EnumDeclaration(&'a EnumDeclaration),
    StructDeclaration(&'a StructDeclaration),
    FunctionDeclaration(&'a Function),
    VariableDeclaration(&'a VariableDeclaration),

    // expression related
    FunctionExpression(&'a Function),
    ArrayExpression(&'a ArrayExpression),
    ParenthesizedExpression(&'a ParenthesizedExpression),
    ConstructionExpression(&'a ConstructionExpression),
    StructConstructionExpression(&'a StructConstructionExpression),
    If(&'a If),
    Else(&'a Else),

    // function inner nodes
    FunctionSignature(&'a FunctionSignature),
    FunctionParameters(&'a FunctionParameters),
    FunctionParameter(&'a FunctionParameter),
    FunctionBody(&'a FunctionBody),

    EnumVariant(&'a EnumVariant),

    StructField(&'a StructField),

    VisibilityModifier(&'a VisibilityModifier),

    // literals
    NumberLiteral(&'a NumberLiteral),
    StringLiteral(&'a StringLiteral),
    BooleanLiteral(&'a BooleanLiteral),

    Identifier(&'a Identifier),

    UnaryOperator(&'a UnaryOperator),
    BinaryOperator(&'a BinaryOperator),
}
