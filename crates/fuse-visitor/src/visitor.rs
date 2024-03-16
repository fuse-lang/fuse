// based on https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html
// and https://github.com/rust-lang/rust/blob/5bc7b9ac8ace5312e1d2cdc2722715cf58d4f926/compiler/rustc_ast_ir/src/visit.rs

use crate::{visit, visit_list, visit_scope, NodeVisitor, ScopeVisitor};
use fuse_ast::{ast::*, AstNode};

pub trait Visitor<'ast>: ScopeVisitor + NodeVisitor + Sized {
    fn visit_chunk(&mut self, chunk: &'ast Chunk) {
        visit_scope!(self => {
            let node = AstNode::Chunk(chunk);
            self.enter_node(node);
            walk_chunk(self, chunk);
            self.leave_node(node);
        });
    }

    fn visit_block(&mut self, block: &'ast Block) {
        visit_scope!(self => {
            let node = AstNode::Block(block);
            self.enter_node(node);
            walk_block(self, block) ;
            self.leave_node(node);
        });
    }

    fn visit_statement(&mut self, statement: &'ast Statement) {
        walk_statement(self, statement)
    }

    fn visit_variable_declaration(&mut self, decl: &'ast VariableDeclaration) {
        let node = AstNode::VariableDeclaration(decl);
        self.enter_node(node);
        walk_variable_declaration(self, decl);
        self.leave_node(node);
    }

    fn visit_function_declaration(&mut self, decl: &'ast Function) {
        visit_scope!(self => {
            let node = AstNode::FunctionDeclaration(decl);
            self.enter_node(node);
            walk_function(self, decl);
            self.leave_node(node);
        });
    }

    fn visit_enum_declaration(&mut self, decl: &'ast EnumDeclaration) {
        visit_scope!(self => {
            let node = AstNode::EnumDeclaration(decl);
            self.enter_node(node);
            walk_enum_declaration(self, decl);
            self.leave_node(node);
        });
    }

    fn visit_enum_variant(&mut self, var: &'ast EnumVariant) {
        let node = AstNode::EnumVariant(var);
        self.enter_node(node);
        walk_enum_variant(self, var);
        self.leave_node(node);
    }

    fn visit_struct_declaration(&mut self, decl: &'ast StructDeclaration) {
        visit_scope!(self => {
            let node = AstNode::StructDeclaration(decl);
            self.enter_node(node);
            walk_struct_declaration(self, decl);
            self.leave_node(node);
        });
    }

    fn visit_struct_field(&mut self, field: &'ast StructField) {
        let node = AstNode::StructField(field);
        self.enter_node(node);
        walk_struct_field(self, field);
        self.leave_node(node);
    }

    fn visit_visibility_modifier(&mut self, vis: &'ast VisibilityModifier) {
        let node = AstNode::VisibilityModifier(vis);
        self.enter_node(node);
        self.leave_node(node);
    }

    fn visit_expression(&mut self, expression: &'ast Expression) {
        walk_expression(self, expression)
    }

    fn visit_number_literal(&mut self, lit: &'ast NumberLiteral) {
        let node = AstNode::NumberLiteral(lit);
        self.enter_node(node);
        self.leave_node(node);
    }

    fn visit_string_literal(&mut self, lit: &'ast StringLiteral) {
        let node = AstNode::StringLiteral(lit);
        self.enter_node(node);
        self.leave_node(node);
    }

    fn visit_boolean_literal(&mut self, lit: &'ast BooleanLiteral) {
        let node = AstNode::BooleanLiteral(lit);
        self.enter_node(node);
        self.leave_node(node);
    }

    fn visit_identifier(&mut self, ident: &'ast Identifier) {
        let node = AstNode::Identifier(ident);
        self.enter_node(node);
        self.leave_node(node);
    }

    fn visit_function_expression(&mut self, func: &'ast Function) {
        visit_scope!(self => {
            let node = AstNode::FunctionExpression(func);
            self.enter_node(node);
            walk_function(self, func);
            self.leave_node(node);
        });
    }

    fn visit_function_signature(&mut self, sign: &'ast FunctionSignature) {
        let node = AstNode::FunctionSignature(sign);
        self.enter_node(node);
        walk_function_signature(self, sign);
        self.leave_node(node);
    }

    fn visit_function_parameters(&mut self, params: &'ast FunctionParameters) {
        let node = AstNode::FunctionParameters(params);
        self.enter_node(node);
        walk_function_parameters(self, params);
        self.leave_node(node);
    }

    fn visit_function_parameter(&mut self, param: &'ast FunctionParameter) {
        let node = AstNode::FunctionParameter(param);
        self.enter_node(node);
        walk_function_parameter(self, param);
        self.leave_node(node);
    }

    fn visit_function_body(&mut self, body: &'ast FunctionBody) {
        let node = AstNode::FunctionBody(body);
        self.enter_node(node);
        walk_function_body(self, body);
        self.leave_node(node);
    }

    fn visit_if(&mut self, r#if: &'ast If) {
        visit_scope!(self => {
            let node = AstNode::If(r#if);
            self.enter_node(node);
            walk_if(self, r#if);
            self.leave_node(node);
        });
    }

    fn visit_else(&mut self, r#else: &'ast Else) {
        visit_scope!(self => {
            let node = AstNode::Else(r#else);
            self.enter_node(node);
            walk_else(self, r#else);
            self.leave_node(node);
        });
    }

    fn visit_unary_operator(&mut self, op: &'ast UnaryOperator) {
        let node = AstNode::UnaryOperator(op);
        self.enter_node(node);
        walk_unary_operator(self, op);
        self.leave_node(node);
    }

    fn visit_binary_operator(&mut self, op: &'ast BinaryOperator) {
        let node = AstNode::BinaryOperator(op);
        self.enter_node(node);
        walk_binary_operator(self, op);
        self.leave_node(node);
    }

    fn visit_array_expression(&mut self, array: &'ast ArrayExpression) {
        let node = AstNode::ArrayExpression(array);
        self.enter_node(node);
        walk_array_expression(self, array);
        self.leave_node(node);
    }

    fn visit_parenthesized_expression(&mut self, expr: &'ast ParenthesizedExpression) {
        let node = AstNode::ParenthesizedExpression(expr);
        self.enter_node(node);
        walk_parenthesized_expression(self, expr);
        self.leave_node(node);
    }

    fn visit_table_construction_expression(&mut self, expr: &'ast ConstructionExpression) {
        let node = AstNode::TableConstructionExpression(expr);
        self.enter_node(node);
        walk_construction_expression(self, expr);
        self.leave_node(node);
    }

    fn visit_struct_construction_expression(&mut self, expr: &'ast StructConstructionExpression) {
        let node = AstNode::StructConstructionExpression(expr);
        self.enter_node(node);
        walk_struct_construction_expression(self, expr);
        self.leave_node(node);
    }

    fn visit_member_expression(&mut self, expr: &'ast MemberExpression) {
        let node = AstNode::MemberExpression(expr);
        self.enter_node(node);
        walk_member_expression(self, expr);
        self.leave_node(node);
    }

    fn visit_member_expression_lhs(&mut self, lhs: &'ast MemberExpressionLHS) {
        let node = AstNode::MemberExpressionLHS(lhs);
        self.enter_node(node);
        walk_member_expression_lhs(self, lhs);
        self.leave_node(node);
    }

    fn visit_member_expression_rhs(&mut self, rhs: &'ast MemberExpressionRHS) {
        let node = AstNode::MemberExpressionRHS(rhs);
        self.enter_node(node);
        walk_member_expression_rhs(self, rhs);
        self.leave_node(node);
    }

    fn visit_construction_expression(&mut self, expr: &'ast ConstructionExpression) {
        let node = AstNode::ConstructionExpression(expr);
        self.enter_node(node);
        walk_construction_expression(self, expr);
        self.leave_node(node);
    }

    fn visit_construction_field(&mut self, field: &'ast ConstructionField) {
        let node = AstNode::ConstructionField(field);
        self.enter_node(node);
        walk_construction_field(self, field);
        self.leave_node(node);
    }

    fn visit_call_expression(&mut self, call: &'ast CallExpression) {
        let node = AstNode::CallExpression(call);
        self.enter_node(node);
        walk_call_expression(self, call);
        self.leave_node(node);
    }

    fn visit_array_expression_element(&mut self, element: &'ast ArrayExpressionElement) {
        let node = AstNode::ArrayExpressionElement(element);
        self.enter_node(node);
        walk_array_expression_element(self, element);
        self.leave_node(node);
    }

    fn visit_binding_pattern(&mut self, pattern: &'ast BindingPattern) {
        let node = AstNode::BindingPattern(pattern);
        self.enter_node(node);
        walk_binding_pattern(self, pattern);
        self.leave_node(node);
    }

    fn visit_binding_identifier(&mut self, binding: &'ast BindingIdentifier) {
        let node = AstNode::BindingIdentifier(binding);
        self.enter_node(node);
        walk_binding_identifier(self, binding);
        self.leave_node(node);
    }

    fn visit_binding_rest(&mut self, arg: &'ast BindingRest) {
        let node = AstNode::BindingRest(arg);
        self.enter_node(node);
        walk_binding_rest(self, arg);
        self.leave_node(node);
    }

    fn visit_key_value_argument(&mut self, arg: &'ast KeyValueArgument) {
        let node = AstNode::KeyValueArgument(arg);
        self.enter_node(node);
        walk_key_value_argument(self, arg);
        self.leave_node(node);
    }

    fn visit_spread_argument(&mut self, arg: &'ast SpreadArgument) {
        let node = AstNode::SpreadArgument(arg);
        self.enter_node(node);
        walk_spread_argument(self, arg);
        self.leave_node(node);
    }

    fn visit_type_annotation(&mut self, annotation: &'ast TypeAnnotation) {
        let node = AstNode::TypeAnnotation(annotation);
        self.enter_node(node);
        walk_type_annotation(self, annotation);
        self.leave_node(node);
    }
}

pub fn walk_chunk<'ast, V: Visitor<'ast>>(visitor: &mut V, chunk: &'ast Chunk) {
    walk_block(visitor, &chunk.body);
}

pub fn walk_block<'ast, V: Visitor<'ast>>(visitor: &mut V, block: &'ast Block) {
    visit_list!(visitor.visit_statement(&block.statements));
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
        Expression::Function(expr) => visit!(visitor.visit_function_expression(expr)),
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
    visit!(visitor.visit_binding_pattern(&param.pattern));
}

pub fn walk_function_body<'ast, V: Visitor<'ast>>(visitor: &mut V, body: &'ast FunctionBody) {
    match body {
        FunctionBody::Block(block) => walk_block(visitor, block),
        FunctionBody::Expression(expr) => visit!(visitor.visit_expression(expr)),
    }
}

pub fn walk_enum_declaration<'ast, V: Visitor<'ast>>(visitor: &mut V, decl: &'ast EnumDeclaration) {
    visit!(visitor.visit_identifier(&decl.identifier));
    visit_list!(visitor.visit_enum_variant(&decl.variants));
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
    visit!(visitor.visit_identifier(&decl.identifier));
    visit_list!(visitor.visit_struct_field(&decl.fields));
}

pub fn walk_struct_field<'ast, V: Visitor<'ast>>(visitor: &mut V, decl: &'ast StructField) {
    visit!(visitor.visit_visibility_modifier(&decl.modifier));
    visit!(visitor.visit_identifier(&decl.identifier));
    visit!(visitor.visit_type_annotation(&decl.type_annotation));
}

pub fn walk_if<'ast, V: Visitor<'ast>>(visitor: &mut V, r#if: &'ast If) {
    visit!(visitor.visit_expression(&r#if.cond));
    visit!(visitor.visit_block(&r#if.body));
    if let Some(r#else) = &r#if.r#else {
        visit!(visitor.visit_else(r#else));
    }
}

pub fn walk_else<'ast, V: Visitor<'ast>>(visitor: &mut V, r#else: &'ast Else) {
    match r#else {
        Else::If(r#if) => visit!(visitor.visit_if(r#if)),
        Else::Block(block) => visit!(visitor.visit_block(block)),
    }
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
    binding: &'ast BindingIdentifier,
) {
    visit!(visitor.visit_identifier(&binding.identifier))
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
