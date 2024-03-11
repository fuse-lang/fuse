use crate::{visit, visit_list, visit_scope};
use fuse_ast::ast::*;

pub trait VisitorMut<'ast>: Sized {
    fn enter_scope(&mut self) {}

    fn leave_scope(&mut self) {}

    fn visit_chunk_mut(&mut self, chunk: &'ast mut Chunk) {
        walk_block_mut(self, &mut chunk.body)
    }

    fn visit_block_mut(&mut self, block: &'ast mut Block) {
        walk_block_mut(self, block)
    }

    fn visit_statement_mut(&mut self, statement: &'ast mut Statement) {
        walk_statement_mut(self, statement)
    }

    fn visit_variable_declaration_mut(&mut self, decl: &'ast mut VariableDeclaration) {
        walk_variable_declaration_mut(self, decl)
    }

    fn visit_function_declaration_mut(&mut self, decl: &'ast mut Function) {
        walk_function_mut(self, decl)
    }

    fn visit_enum_declaration_mut(&mut self, decl: &'ast mut EnumDeclaration) {
        walk_enum_declaration_mut(self, decl)
    }

    fn visit_enum_variant_mut(&mut self, var: &'ast mut EnumVariant) {
        walk_enum_variant_mut(self, var)
    }

    fn visit_struct_declaration_mut(&mut self, decl: &'ast mut StructDeclaration) {
        walk_struct_declaration_mut(self, decl)
    }

    fn visit_struct_field_mut(&mut self, field: &'ast mut StructField) {
        walk_struct_field_mut(self, field)
    }

    fn visit_visibility_modifier_mut(&mut self, _: &'ast mut VisibilityModifier) {}

    fn visit_expression_mut(&mut self, expression: &'ast mut Expression) {
        walk_expression_mut(self, expression)
    }

    fn visit_number_literal_mut(&mut self, _: &'ast mut NumberLiteral) {}

    fn visit_string_literal_mut(&mut self, _: &'ast mut StringLiteral) {}

    fn visit_boolean_literal_mut(&mut self, _: &'ast mut BooleanLiteral) {}

    fn visit_identifier_mut(&mut self, _: &'ast mut Identifier) {}

    fn visit_function_mut(&mut self, func: &'ast mut Function) {
        walk_function_mut(self, func)
    }

    fn visit_function_signature_mut(&mut self, sign: &'ast mut FunctionSignature) {
        walk_function_signature_mut(self, sign)
    }

    fn visit_function_parameters_mut(&mut self, params: &'ast mut FunctionParameters) {
        walk_function_parameters_mut(self, params)
    }

    fn visit_function_parameter_mut(&mut self, param: &'ast mut FunctionParameter) {
        walk_function_parameter_mut(self, param)
    }

    fn visit_function_body_mut(&mut self, body: &'ast mut FunctionBody) {
        walk_function_body_mut(self, body)
    }

    fn visit_if_mut(&mut self, r#if: &'ast mut If) {
        walk_if_mut(self, r#if)
    }

    fn visit_else_mut(&mut self, r#else: &'ast mut Else) {
        walk_else_mut(self, r#else)
    }

    fn visit_unary_operator_mut(&mut self, op: &'ast mut UnaryOperator) {
        walk_unary_operator_mut(self, op)
    }

    fn visit_binary_operator_mut(&mut self, op: &'ast mut BinaryOperator) {
        walk_binary_operator_mut(self, op)
    }

    fn visit_array_expression_mut(&mut self, array: &'ast mut ArrayExpression) {
        walk_array_expression_mut(self, array)
    }

    fn visit_parenthesized_expression_mut(&mut self, expr: &'ast mut ParenthesizedExpression) {
        walk_parenthesized_expression_mut(self, expr)
    }

    fn visit_table_construction_expression_mut(&mut self, expr: &'ast mut ConstructionExpression) {
        walk_construction_expression_mut(self, expr)
    }

    fn visit_struct_construction_expression_mut(
        &mut self,
        expr: &'ast mut StructConstructionExpression,
    ) {
        walk_struct_construction_expression_mut(self, expr)
    }

    fn visit_construction_expression_mut(&mut self, expr: &'ast mut ConstructionExpression) {
        walk_construction_expression_mut(self, expr)
    }

    fn visit_construction_field_mut(&mut self, field: &'ast mut ConstructionField) {
        walk_construction_field_mut(self, field)
    }

    fn visit_call_expression_mut(&mut self, call: &'ast mut CallExpression) {
        walk_call_expression_mut(self, call)
    }

    fn visit_array_expression_element_mut(&mut self, element: &'ast mut ArrayExpressionElement) {
        walk_array_expression_element_mut(self, element)
    }

    fn visit_binding_pattern_mut(&mut self, pattern: &'ast mut BindingPattern) {
        walk_binding_pattern_mut(self, pattern)
    }

    fn visit_binding_identifier_mut(&mut self, pattern: &'ast mut BindingIdentifier) {
        walk_binding_identifier_mut(self, pattern)
    }

    fn visit_binding_rest_mut(&mut self, arg: &'ast mut BindingRest) {
        walk_binding_rest_mut(self, arg)
    }

    fn visit_key_value_argument_mut(&mut self, arg: &'ast mut KeyValueArgument) {
        walk_key_value_argument_mut(self, arg)
    }

    fn visit_spread_argument_mut(&mut self, arg: &'ast mut SpreadArgument) {
        walk_spread_argument_mut(self, arg)
    }

    fn visit_type_annotation_mut(&mut self, annotation: &'ast mut TypeAnnotation) {
        walk_type_annotation_mut(self, annotation)
    }
}

pub fn walk_block_mut<'ast, V: VisitorMut<'ast>>(visitor: &mut V, block: &'ast mut Block) {
    visit_scope!(visitor => {
        visit_list!(visitor.visit_statement_mut(&mut block.statements));
    });
}

pub fn walk_statement_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    statement: &'ast mut Statement,
) {
    match statement {
        Statement::Empty(_) => {}
        Statement::Expression(expr) => visit!(visitor.visit_expression_mut(expr)),
        Statement::VariableDeclaration(decl) => {
            visit!(visitor.visit_variable_declaration_mut(decl))
        }
        Statement::FunctionDeclaration(func) => {
            visit!(visitor.visit_function_declaration_mut(func))
        }
        Statement::EnumDeclaration(decl) => visit!(visitor.visit_enum_declaration_mut(decl)),
        // Statement::StructDeclaration(Box<StructDeclaration>),
        // Statement::ImplStatement(Box<ImplStatement>),
        _ => todo!(),
    }
}

pub fn walk_variable_declaration_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    decl: &'ast mut VariableDeclaration,
) {
    visit!(visitor.visit_binding_pattern_mut(&mut decl.binding));
    if let Some(decl) = &mut decl.expression {
        visit!(visitor.visit_expression_mut(decl));
    }
}

pub fn walk_expression_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    expression: &'ast mut Expression,
) {
    match expression {
        Expression::NumberLiteral(expr) => visit!(visitor.visit_number_literal_mut(expr)),
        Expression::StringLiteral(expr) => visit!(visitor.visit_string_literal_mut(expr)),
        Expression::BooleanLiteral(expr) => visit!(visitor.visit_boolean_literal_mut(expr)),
        Expression::Identifier(expr) => visit!(visitor.visit_identifier_mut(expr)),
        Expression::Function(expr) => visit!(visitor.visit_function_mut(expr)),
        Expression::If(expr) => visit!(visitor.visit_if_mut(expr)),
        Expression::UnaryOperator(expr) => visit!(visitor.visit_unary_operator_mut(expr)),
        Expression::BinaryOperator(expr) => visit!(visitor.visit_binary_operator_mut(expr)),
        Expression::ArrayExpression(expr) => visit!(visitor.visit_array_expression_mut(expr)),
        Expression::TupleExpression(..) => todo!(),
        Expression::ParenthesizedExpression(expr) => {
            visit!(visitor.visit_parenthesized_expression_mut(expr))
        }
        Expression::CallExpression(expr) => visit!(visitor.visit_call_expression_mut(expr)),
        Expression::TableConstructionExpression(expr) => {
            visit!(visitor.visit_table_construction_expression_mut(expr))
        }
        Expression::StructConstructionExpression(expr) => {
            visit!(visitor.visit_struct_construction_expression_mut(expr))
        }
    }
}

pub fn walk_function_mut<'ast, V: VisitorMut<'ast>>(visitor: &mut V, func: &'ast mut Function) {
    visit_scope!(visitor => {
        visit!(visitor.visit_function_signature_mut(&mut func.signature));
        visit!(visitor.visit_function_body_mut(&mut func.body));
    });
}

pub fn walk_function_signature_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    sign: &'ast mut FunctionSignature,
) {
    if let Some(ident) = &mut sign.identifier {
        visit!(visitor.visit_identifier_mut(ident));
    }
    visit!(visitor.visit_function_parameters_mut(&mut sign.params));
    if let Some(annotation) = &mut sign.return_type {
        visit!(visitor.visit_type_annotation_mut(annotation));
    }
}

pub fn walk_function_parameters_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    params: &'ast mut FunctionParameters,
) {
    for param in &mut params.items {
        visit!(visitor.visit_function_parameter_mut(param))
    }
    if let Some(rest) = &mut params.rest {
        visit!(visitor.visit_binding_rest_mut(rest))
    }
}

pub fn walk_function_parameter_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    param: &'ast mut FunctionParameter,
) {
    visit!(visitor.visit_binding_pattern_mut(&mut param.pattern))
}

pub fn walk_function_body_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    body: &'ast mut FunctionBody,
) {
    match body {
        FunctionBody::Block(block) => visit!(visitor.visit_block_mut(block)),
        FunctionBody::Expression(expr) => visit!(visitor.visit_expression_mut(expr)),
    }
}

pub fn walk_enum_declaration_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    decl: &'ast mut EnumDeclaration,
) {
    visit_scope!(visitor => {
        visit!(visitor.visit_identifier_mut(&mut decl.identifier));
        visit_list!(visitor.visit_enum_variant_mut(&mut decl.variants));
    });
}

pub fn walk_enum_variant_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    var: &'ast mut EnumVariant,
) {
    visit!(visitor.visit_identifier_mut(&mut var.identifier));
    if let Some(value) = &mut var.value {
        visit!(visitor.visit_expression_mut(value));
    }
}

pub fn walk_struct_declaration_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    decl: &'ast mut StructDeclaration,
) {
    visit_scope!(visitor => {
        visit!(visitor.visit_identifier_mut(&mut decl.identifier));
        visit_list!(visitor.visit_struct_field_mut(&mut decl.fields));
    });
}

pub fn walk_struct_field_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    decl: &'ast mut StructField,
) {
    visit!(visitor.visit_visibility_modifier_mut(&mut decl.modifier));
    visit!(visitor.visit_identifier_mut(&mut decl.identifier));
    visit!(visitor.visit_type_annotation_mut(&mut decl.type_annotation));
}

pub fn walk_if_mut<'ast, V: VisitorMut<'ast>>(visitor: &mut V, r#if: &'ast mut If) {
    visit_scope!(visitor => {
        visit!(visitor.visit_expression_mut(&mut r#if.cond));
        visit!(visitor.visit_block_mut(&mut r#if.body));
        if let Some(r#else) = &mut r#if.r#else {
            visit!(visitor.visit_else_mut(r#else));
        }
    });
}

pub fn walk_else_mut<'ast, V: VisitorMut<'ast>>(visitor: &mut V, r#else: &'ast mut Else) {
    visit_scope!(visitor => {
        match r#else {
            Else::If(r#if) => visit!(visitor.visit_if_mut(r#if)),
            Else::Block(block) => visit!(visitor.visit_block_mut(block)),
        }
    });
}

pub fn walk_unary_operator_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    op: &'ast mut UnaryOperator,
) {
    visit!(visitor.visit_expression_mut(&mut op.expression))
}

pub fn walk_binary_operator_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    op: &'ast mut BinaryOperator,
) {
    visit!(visitor.visit_expression_mut(&mut op.lhs));
    visit!(visitor.visit_expression_mut(&mut op.rhs));
}

pub fn walk_array_expression_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    array: &'ast mut ArrayExpression,
) {
    visit_list!(visitor.visit_array_expression_element_mut(&mut array.elements))
}

pub fn walk_parenthesized_expression_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    expr: &'ast mut ParenthesizedExpression,
) {
    visit!(visitor.visit_expression_mut(&mut expr.expression))
}

pub fn walk_struct_construction_expression_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    expr: &'ast mut StructConstructionExpression,
) {
    visit!(visitor.visit_expression_mut(&mut expr.target));
    visit!(visitor.visit_construction_expression_mut(&mut expr.construction));
}

pub fn walk_construction_expression_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    expr: &'ast mut ConstructionExpression,
) {
    visit_list!(visitor.visit_construction_field_mut(&mut expr.fields))
}

pub fn walk_construction_field_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    field: &'ast mut ConstructionField,
) {
    match field {
        ConstructionField::Expression(expr) => visit!(visitor.visit_expression_mut(expr)),
        ConstructionField::KeyValueArgument(kv) => visit!(visitor.visit_key_value_argument_mut(kv)),
        ConstructionField::Spread(spread) => visit!(visitor.visit_spread_argument_mut(spread)),
    }
}

pub fn walk_call_expression_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    call: &'ast mut CallExpression,
) {
    visit!(visitor.visit_expression_mut(&mut call.callee));
    visit_list!(visitor.visit_expression_mut(&mut call.arguments));
}

pub fn walk_array_expression_element_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    element: &'ast mut ArrayExpressionElement,
) {
    match element {
        ArrayExpressionElement::Expression(expr) => visit!(visitor.visit_expression_mut(expr)),
        ArrayExpressionElement::Spread(spread) => visit!(visitor.visit_spread_argument_mut(spread)),
    }
}

pub fn walk_binding_pattern_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    pattern: &'ast mut BindingPattern,
) {
    match &mut pattern.kind {
        BindingPatternKind::Identifier(patt) => {
            visit!(visitor.visit_binding_identifier_mut(patt))
        }
        _ => todo!(),
    }
}

pub fn walk_binding_identifier_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    pattern: &'ast mut BindingIdentifier,
) {
    visit!(visitor.visit_identifier_mut(&mut pattern.identifier))
}

pub fn walk_binding_rest_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    rest: &'ast mut BindingRest,
) {
    visit!(visitor.visit_binding_identifier_mut(&mut rest.binding));
    if let Some(annotation) = &mut rest.type_annotation {
        visit!(visitor.visit_type_annotation_mut(annotation))
    };
}

pub fn walk_key_value_argument_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    kv: &'ast mut KeyValueArgument,
) {
    visit!(visitor.visit_identifier_mut(&mut kv.key));
    visit!(visitor.visit_expression_mut(&mut kv.value));
}

pub fn walk_spread_argument_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    spread: &'ast mut SpreadArgument,
) {
    visit!(visitor.visit_expression_mut(&mut spread.element));
}

pub fn walk_type_annotation_mut<'ast, V: VisitorMut<'ast>>(
    visitor: &mut V,
    annotation: &'ast mut TypeAnnotation,
) {
    println!("not implemented.")
}
