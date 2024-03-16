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
    CallExpression(&'a CallExpression),
    ArrayExpression(&'a ArrayExpression),
    ParenthesizedExpression(&'a ParenthesizedExpression),
    ConstructionExpression(&'a ConstructionExpression),
    TableConstructionExpression(&'a ConstructionExpression),
    StructConstructionExpression(&'a StructConstructionExpression),
    If(&'a If),
    Else(&'a Else),

    // function inner nodes
    FunctionSignature(&'a FunctionSignature),
    FunctionParameters(&'a FunctionParameters),
    FunctionParameter(&'a FunctionParameter),
    FunctionBody(&'a FunctionBody),

    // Misc
    EnumVariant(&'a EnumVariant),
    StructField(&'a StructField),
    ConstructionField(&'a ConstructionField),
    ArrayExpressionElement(&'a ArrayExpressionElement),

    VisibilityModifier(&'a VisibilityModifier),

    // literals
    NumberLiteral(&'a NumberLiteral),
    StringLiteral(&'a StringLiteral),
    BooleanLiteral(&'a BooleanLiteral),

    Identifier(&'a Identifier),
    BindingPattern(&'a BindingPattern),
    BindingIdentifier(&'a BindingIdentifier),
    BindingRest(&'a BindingRest),
    KeyValueArgument(&'a KeyValueArgument),
    SpreadArgument(&'a SpreadArgument),

    UnaryOperator(&'a UnaryOperator),
    BinaryOperator(&'a BinaryOperator),

    MemberExpression(&'a MemberExpression),
    MemberExpressionLHS(&'a MemberExpressionLHS),
    MemberExpressionRHS(&'a MemberExpressionRHS),

    TypeAnnotation(&'a TypeAnnotation),
}
