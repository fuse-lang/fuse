// based on https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
// and https://github.com/rust-lang/rust/blob/5bc7b9ac8ace5312e1d2cdc2722715cf58d4f926/compiler/rustc_ast_ir/src/visit.rs

use crate::{visit, visit_list, visit_scope, ScopeVisitor};
use fuse_ast::ast::*;

pub trait Visitor<'ast>: ScopeVisitor + Sized {
    fn visit_chunk(&mut self, chunk: &'ast Chunk) {
        walk_block(self, &chunk.body)
    }

    fn visit_block(&mut self, block: &'ast Block) {
        walk_block(self, block)
    }

    fn visit_statement(&mut self, statement: &'ast Statement) {
        walk_statement(self, statement)
    }

    fn visit_variable_declaration(&mut self, decl: &'ast VariableDeclaration) {
        walk_variable_declaration(self, decl)
    }

    fn visit_function_declaration(&mut self, decl: &'ast Function) {
        walk_function(self, decl)
    }

    fn visit_enum_declaration(&mut self, decl: &'ast EnumDeclaration) {
        walk_enum_declaration(self, decl)
    }

    fn visit_enum_variant(&mut self, var: &'ast EnumVariant) {
        walk_enum_variant(self, var)
    }

    fn visit_struct_declaration(&mut self, decl: &'ast StructDeclaration) {
        walk_struct_declaration(self, decl)
    }

    fn visit_struct_field(&mut self, field: &'ast StructField) {
        walk_struct_field(self, field)
    }

    fn visit_visibility_modifier(&mut self, _: &'ast VisibilityModifier) {}

    fn visit_expression(&mut self, expression: &'ast Expression) {
        walk_expression(self, expression)
    }

    fn visit_number_literal(&mut self, _: &'ast NumberLiteral) {}

    fn visit_string_literal(&mut self, _: &'ast StringLiteral) {}

    fn visit_boolean_literal(&mut self, _: &'ast BooleanLiteral) {}

    fn visit_identifier(&mut self, _: &'ast Identifier) {}

    fn visit_function(&mut self, func: &'ast Function) {
        walk_function(self, func)
    }

    fn visit_function_signature(&mut self, sign: &'ast FunctionSignature) {
        walk_function_signature(self, sign)
    }

    fn visit_function_parameters(&mut self, params: &'ast FunctionParameters) {
        walk_function_parameters(self, params)
    }

    fn visit_function_parameter(&mut self, param: &'ast FunctionParameter) {
        walk_function_parameter(self, param)
    }

    fn visit_function_body(&mut self, body: &'ast FunctionBody) {
        walk_function_body(self, body)
    }

    fn visit_if(&mut self, r#if: &'ast If) {
        walk_if(self, r#if)
    }

    fn visit_else(&mut self, r#else: &'ast Else) {
        walk_else(self, r#else)
    }

    fn visit_unary_operator(&mut self, op: &'ast UnaryOperator) {
        walk_unary_operator(self, op)
    }

    fn visit_binary_operator(&mut self, op: &'ast BinaryOperator) {
        walk_binary_operator(self, op)
    }

    fn visit_array_expression(&mut self, array: &'ast ArrayExpression) {
        walk_array_expression(self, array)
    }

    fn visit_parenthesized_expression(&mut self, expr: &'ast ParenthesizedExpression) {
        walk_parenthesized_expression(self, expr)
    }

    fn visit_table_construction_expression(&mut self, expr: &'ast ConstructionExpression) {
        walk_construction_expression(self, expr)
    }

    fn visit_struct_construction_expression(&mut self, expr: &'ast StructConstructionExpression) {
        walk_struct_construction_expression(self, expr)
    }

    fn visit_member_expression(&mut self, expr: &'ast MemberExpression) {
        walk_member_expression(self, expr)
    }

    fn visit_member_expression_lhs(&mut self, lhs: &'ast MemberExpressionLHS) {
        walk_member_expression_lhs(self, lhs)
    }

    fn visit_member_expression_rhs(&mut self, rhs: &'ast MemberExpressionRHS) {
        walk_member_expression_rhs(self, rhs)
    }

    fn visit_construction_expression(&mut self, expr: &'ast ConstructionExpression) {
        walk_construction_expression(self, expr)
    }

    fn visit_construction_field(&mut self, field: &'ast ConstructionField) {
        walk_construction_field(self, field)
    }

    fn visit_call_expression(&mut self, call: &'ast CallExpression) {
        walk_call_expression(self, call)
    }

    fn visit_array_expression_element(&mut self, element: &'ast ArrayExpressionElement) {
        walk_array_expression_element(self, element)
    }

    fn visit_binding_pattern(&mut self, pattern: &'ast BindingPattern) {
        walk_binding_pattern(self, pattern)
    }

    fn visit_binding_identifier(&mut self, pattern: &'ast BindingIdentifier) {
        walk_binding_identifier(self, pattern)
    }

    fn visit_binding_rest(&mut self, arg: &'ast BindingRest) {
        walk_binding_rest(self, arg)
    }

    fn visit_key_value_argument(&mut self, arg: &'ast KeyValueArgument) {
        walk_key_value_argument(self, arg)
    }

    fn visit_spread_argument(&mut self, arg: &'ast SpreadArgument) {
        walk_spread_argument(self, arg)
    }

    fn visit_type_annotation(&mut self, annotation: &'ast TypeAnnotation) {
        walk_type_annotation(self, annotation)
    }
}

pub fn walk_block<'ast, V: Visitor<'ast>>(visitor: &mut V, block: &'ast Block) {
    visit_scope!(visitor => {
        visit_list!(visitor.visit_statement(&block.statements));
    });
}

pub fn walk_statement<'ast, V: Visitor<'ast>>(visitor: &mut V, statement: &'ast Statement) {
    match statement {
        Statement::Empty(_) => {}
        Statement::Expression(expr) => visit!(visitor.visit_expression(expr)),
        Statement::VariableDeclaration(decl) => visit!(visitor.visit_variable_declaration(decl)),
        Statement::FunctionDeclaration(func) => visit!(visitor.visit_function_declaration(func)),
        Statement::EnumDeclaration(decl) => visit!(visitor.visit_enum_declaration(decl)),
        // Statement::StructDeclaration(Box<StructDeclaration>),
        // Statement::ImplStatement(Box<ImplStatement>),
        _ => todo!(),
    }
}

pub fn walk_variable_declaration<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    decl: &'ast VariableDeclaration,
) {
    visit!(visitor.visit_binding_pattern(&decl.binding));
    if let Some(decl) = &decl.expression {
        visit!(visitor.visit_expression(decl));
    }
}

pub fn walk_expression<'ast, V: Visitor<'ast>>(visitor: &mut V, expression: &'ast Expression) {
    match expression {
        Expression::NumberLiteral(expr) => visit!(visitor.visit_number_literal(expr)),
        Expression::StringLiteral(expr) => visit!(visitor.visit_string_literal(expr)),
        Expression::BooleanLiteral(expr) => visit!(visitor.visit_boolean_literal(expr)),
        Expression::Identifier(expr) => visit!(visitor.visit_identifier(expr)),
        Expression::Function(expr) => visit!(visitor.visit_function(expr)),
        Expression::If(expr) => visit!(visitor.visit_if(expr)),
        Expression::UnaryOperator(expr) => visit!(visitor.visit_unary_operator(expr)),
        Expression::BinaryOperator(expr) => visit!(visitor.visit_binary_operator(expr)),
        Expression::ArrayExpression(expr) => visit!(visitor.visit_array_expression(expr)),
        Expression::TupleExpression(..) => todo!(),
        Expression::ParenthesizedExpression(expr) => {
            visit!(visitor.visit_parenthesized_expression(expr))
        }
        Expression::CallExpression(expr) => visit!(visitor.visit_call_expression(expr)),
        Expression::TableConstructionExpression(expr) => {
            visit!(visitor.visit_table_construction_expression(expr))
        }
        Expression::StructConstructionExpression(expr) => {
            visit!(visitor.visit_struct_construction_expression(expr))
        }
        Expression::MemberExpression(expr) => {
            visit!(visitor.visit_member_expression(expr))
        }
    }
}

pub fn walk_function<'ast, V: Visitor<'ast>>(visitor: &mut V, func: &'ast Function) {
    visit_scope!(visitor => {
        visit!(visitor.visit_function_signature(&func.signature));
        visit!(visitor.visit_function_body(&func.body));
    });
}

pub fn walk_function_signature<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    sign: &'ast FunctionSignature,
) {
    if let Some(ident) = &sign.identifier {
        visit!(visitor.visit_identifier(ident));
    }
    visit!(visitor.visit_function_parameters(&sign.params));
    if let Some(annotation) = &sign.return_type {
        visit!(visitor.visit_type_annotation(annotation));
    }
}

pub fn walk_function_parameters<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    params: &'ast FunctionParameters,
) {
    for param in &params.items {
        visit!(visitor.visit_function_parameter(param))
    }
    if let Some(rest) = &params.rest {
        visit!(visitor.visit_binding_rest(rest))
    }
}

pub fn walk_function_parameter<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    param: &'ast FunctionParameter,
) {
    visit!(visitor.visit_binding_pattern(&param.pattern))
}

pub fn walk_function_body<'ast, V: Visitor<'ast>>(visitor: &mut V, body: &'ast FunctionBody) {
    match body {
        FunctionBody::Block(block) => visit!(visitor.visit_block(block)),
        FunctionBody::Expression(expr) => visit!(visitor.visit_expression(expr)),
    }
}

pub fn walk_enum_declaration<'ast, V: Visitor<'ast>>(visitor: &mut V, decl: &'ast EnumDeclaration) {
    visit_scope!(visitor => {
        visit!(visitor.visit_identifier(&decl.identifier));
        visit_list!(visitor.visit_enum_variant(&decl.variants));
    });
}

pub fn walk_enum_variant<'ast, V: Visitor<'ast>>(visitor: &mut V, var: &'ast EnumVariant) {
    visit!(visitor.visit_identifier(&var.identifier));
    if let Some(value) = &var.value {
        visit!(visitor.visit_expression(value));
    }
}

pub fn walk_struct_declaration<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    decl: &'ast StructDeclaration,
) {
    visit_scope!(visitor => {
        visit!(visitor.visit_identifier(&decl.identifier));
        visit_list!(visitor.visit_struct_field(&decl.fields));
    });
}

pub fn walk_struct_field<'ast, V: Visitor<'ast>>(visitor: &mut V, decl: &'ast StructField) {
    visit!(visitor.visit_visibility_modifier(&decl.modifier));
    visit!(visitor.visit_identifier(&decl.identifier));
    visit!(visitor.visit_type_annotation(&decl.type_annotation));
}

pub fn walk_if<'ast, V: Visitor<'ast>>(visitor: &mut V, r#if: &'ast If) {
    visit_scope!(visitor => {
        visit!(visitor.visit_expression(&r#if.cond));
        visit!(visitor.visit_block(&r#if.body));
        if let Some(r#else) = &r#if.r#else {
            visit!(visitor.visit_else(r#else));
        }
    });
}

pub fn walk_else<'ast, V: Visitor<'ast>>(visitor: &mut V, r#else: &'ast Else) {
    visit_scope!(visitor => {
        match r#else {
            Else::If(r#if) => visit!(visitor.visit_if(r#if)),
            Else::Block(block) => visit!(visitor.visit_block(block)),
        }
    });
}

pub fn walk_unary_operator<'ast, V: Visitor<'ast>>(visitor: &mut V, op: &'ast UnaryOperator) {
    visit!(visitor.visit_expression(&op.expression))
}

pub fn walk_binary_operator<'ast, V: Visitor<'ast>>(visitor: &mut V, op: &'ast BinaryOperator) {
    visit!(visitor.visit_expression(&op.lhs));
    visit!(visitor.visit_expression(&op.rhs));
}

pub fn walk_array_expression<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    array: &'ast ArrayExpression,
) {
    visit_list!(visitor.visit_array_expression_element(&array.elements))
}

pub fn walk_parenthesized_expression<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    expr: &'ast ParenthesizedExpression,
) {
    visit!(visitor.visit_expression(&expr.expression))
}

pub fn walk_struct_construction_expression<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    expr: &'ast StructConstructionExpression,
) {
    visit!(visitor.visit_expression(&expr.target));
    visit!(visitor.visit_construction_expression(&expr.construction));
}

pub fn walk_member_expression<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    expr: &'ast MemberExpression,
) {
    visit!(visitor.visit_member_expression_lhs(&expr.lhs));
    visit!(visitor.visit_member_expression_rhs(&expr.rhs));
}

pub fn walk_member_expression_lhs<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    lhs: &'ast MemberExpressionLHS,
) {
    match lhs {
        MemberExpressionLHS::Identifier(ident) => visit!(visitor.visit_identifier(ident)),
        MemberExpressionLHS::Expression(expr) => visit!(visitor.visit_expression(expr)),
        MemberExpressionLHS::Member(member) => visit!(visitor.visit_member_expression(member)),
        MemberExpressionLHS::Call(call) => visit!(visitor.visit_call_expression(call)),
    }
}

pub fn walk_member_expression_rhs<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    rhs: &'ast MemberExpressionRHS,
) {
    match rhs {
        MemberExpressionRHS::Identifier(ident) => visit!(visitor.visit_identifier(ident)),
        MemberExpressionRHS::Call(call) => visit!(visitor.visit_call_expression(call)),
        MemberExpressionRHS::Number(_) => todo!("support for numeric access to tuple fields."),
        MemberExpressionRHS::Member(member) => visit!(visitor.visit_member_expression(member)),
    }
}

pub fn walk_construction_expression<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    expr: &'ast ConstructionExpression,
) {
    visit_list!(visitor.visit_construction_field(&expr.fields))
}

pub fn walk_construction_field<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    field: &'ast ConstructionField,
) {
    match field {
        ConstructionField::Expression(expr) => visit!(visitor.visit_expression(expr)),
        ConstructionField::KeyValueArgument(kv) => visit!(visitor.visit_key_value_argument(kv)),
        ConstructionField::Spread(spread) => visit!(visitor.visit_spread_argument(spread)),
    }
}

pub fn walk_call_expression<'ast, V: Visitor<'ast>>(visitor: &mut V, call: &'ast CallExpression) {
    visit!(visitor.visit_expression(&call.callee));
    visit_list!(visitor.visit_expression(&call.arguments));
}

pub fn walk_array_expression_element<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    element: &'ast ArrayExpressionElement,
) {
    match element {
        ArrayExpressionElement::Expression(expr) => visit!(visitor.visit_expression(expr)),
        ArrayExpressionElement::Spread(spread) => visit!(visitor.visit_spread_argument(spread)),
    }
}

pub fn walk_binding_pattern<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    pattern: &'ast BindingPattern,
) {
    match &pattern.kind {
        BindingPatternKind::Identifier(patt) => {
            visit!(visitor.visit_binding_identifier(patt))
        }
        _ => todo!(),
    }
}

pub fn walk_binding_identifier<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    pattern: &'ast BindingIdentifier,
) {
    visit!(visitor.visit_identifier(&pattern.identifier))
}

pub fn walk_binding_rest<'ast, V: Visitor<'ast>>(visitor: &mut V, rest: &'ast BindingRest) {
    visit!(visitor.visit_binding_identifier(&rest.binding));
    if let Some(annotation) = &rest.type_annotation {
        visit!(visitor.visit_type_annotation(annotation))
    };
}

pub fn walk_key_value_argument<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    kv: &'ast KeyValueArgument,
) {
    visit!(visitor.visit_identifier(&kv.key));
    visit!(visitor.visit_expression(&kv.value));
}

pub fn walk_spread_argument<'ast, V: Visitor<'ast>>(visitor: &mut V, spread: &'ast SpreadArgument) {
    visit!(visitor.visit_expression(&spread.element));
}

pub fn walk_type_annotation<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    annotation: &'ast TypeAnnotation,
) {
    println!("not implemented.")
}

pub fn walk_template<'ast, V: Visitor<'ast>>(visitor: &mut V, expression: &'ast Expression) {}
