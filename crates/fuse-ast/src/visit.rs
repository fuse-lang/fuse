// based on https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
// and https://github.com/rust-lang/rust/blob/5bc7b9ac8ace5312e1d2cdc2722715cf58d4f926/compiler/rustc_ast_ir/src/visit.rs

use crate::ast::*;

macro_rules! visit {
    ($expr:expr) => {
        $expr
    };
}

macro_rules! visit_list {
    ($visitor:ident, $method:ident, $list:expr $(, $($extra_args:expr), *)?) => {
        for elem in $list {
            visit!($visitor.$method(elem $(, $($extra_args),*)?))
        }
    };
}

pub trait Visitor<'ast>: Sized {
    fn visit_chunk(&mut self, chunk: &'ast Chunk) {
        walk_block(self, &chunk.body)
    }

    fn visit_block(&mut self, block: &'ast Block) {
        walk_block(self, block)
    }

    fn visit_statement(&mut self, statement: &'ast Statement) {
        walk_statement(self, &statement)
    }

    fn visit_expression(&mut self, expression: &'ast Expression) {
        walk_expression(self, &expression)
    }

    fn visit_number_literal(&mut self, _: &'ast NumberLiteral) {}

    fn visit_string_literal(&mut self, _: &'ast StringLiteral) {}

    fn visit_boolean_literal(&mut self, _: &'ast BooleanLiteral) {}

    fn visit_identifier(&mut self, _: &'ast Identifier) {}

    fn visit_function(&mut self, func: &'ast Function) {
        walk_function(self, &func)
    }

    fn visit_function_signature(&mut self, sign: &'ast FunctionSignature) {
        walk_function_signature(self, &sign)
    }

    fn visit_function_parameters(&mut self, params: &'ast FunctionParameters) {
        walk_function_parameters(self, &params)
    }

    fn visit_function_parameter(&mut self, param: &'ast FunctionParameter) {
        walk_function_parameter(self, &param)
    }

    fn visit_function_body(&mut self, body: &'ast FunctionBody) {
        walk_function_body(self, &body)
    }

    fn visit_binding_pattern(&mut self, pattern: &'ast BindingPattern) {
        walk_binding_pattern(self, &pattern)
    }

    fn visit_binding_identifier(&mut self, pattern: &'ast BindingIdentifier) {
        walk_binding_identifier(self, &pattern)
    }

    fn visit_binding_rest(&mut self, rest: &'ast BindingRest) {
        walk_binding_rest(self, &rest)
    }

    fn visit_type_annotation(&mut self, annotation: &'ast TypeAnnotation) {
        walk_type_annotation(self, &annotation)
    }
}

pub fn walk_block<'ast, V: Visitor<'ast>>(visitor: &mut V, block: &'ast Block) {
    visit_list!(visitor, visit_statement, &block.statements)
}

pub fn walk_statement<'ast, V: Visitor<'ast>>(visitor: &mut V, statement: &'ast Statement) {
    match statement {
    Statement::Empty(_) => {},
    Statement::Expression(expr) => visit!(visitor.visit_expression(expr)),
    _ => todo!()
    // Statement::VariableDeclaration(Box<VariableDeclaration>),
    // Statement::FunctionDeclaration(Box<Function>),
    // Statement::EnumDeclaration(Box<EnumDeclaration>),
    // Statement::StructDeclaration(Box<StructDeclaration>),
    // Statement::ImplStatement(Box<ImplStatement>),
    }
}

pub fn walk_expression<'ast, V: Visitor<'ast>>(visitor: &mut V, expression: &'ast Expression) {
    match expression {
        Expression::NumberLiteral(lit) => visit!(visitor.visit_number_literal(lit)),
        Expression::StringLiteral(lit) => visit!(visitor.visit_string_literal(lit)),
        Expression::BooleanLiteral(lit) => visit!(visitor.visit_boolean_literal(lit)),
        Expression::Identifier(ident) => visit!(visitor.visit_identifier(ident)),
        Expression::Function(func) => visit!(visitor.visit_function(func)),

        // Expression::If(e),
        // Expression::UnaryOperator(e),
        // Expression::BinaryOperator(e),
        // Expression::ArrayExpression(e),
        // Expression::TupleExpression(e),
        // Expression::ParenthesizedExpression(e),
        // Expression::CallExpression(e),
        // Expression::TableConstructionExpression(e),
        // Expression::StructConstructionExpression(e),
        _ => todo!(),
    }
}

pub fn walk_function<'ast, V: Visitor<'ast>>(visitor: &mut V, func: &'ast Function) {
    visit!(visitor.visit_function_signature(&func.signature));
    visit!(visitor.visit_function_body(&func.body));
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

pub fn walk_type_annotation<'ast, V: Visitor<'ast>>(
    visitor: &mut V,
    annotation: &'ast TypeAnnotation,
) {
}

pub fn walk_template<'ast, V: Visitor<'ast>>(visitor: &mut V, expression: &'ast Expression) {}
