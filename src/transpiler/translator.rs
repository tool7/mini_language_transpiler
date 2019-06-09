use crate::transpiler::parser::*;

macro_rules! format_function_definition {
    ($name:expr, $args:expr, $return_type:expr, $body:expr) => (
        format!("fn {}({}) -> {} {{\n{}\n}}", $name, $args, $return_type, $body);
    )
}

macro_rules! format_conditional {
    ($if_expr:expr, $then_expr:expr, $else_expr:expr) => {
        format!("if {} {{\n\t{}\n}}\nelse {{\n\t{}\n}}", $if_expr, $then_expr, $else_expr);
    }
}

macro_rules! format_loop {
    () => {

    }
}

pub fn translate(ast: &[ASTNode]) -> Result<String, String> {
    let mut translated_expressions = String::new();
    let mut translated_function_definitions = String::new();

    for node in ast {
        let FunctionNode(function) = node;

        let mut translated_expression: String = translate_expression(&function.body);

        // Not function definition
        if function.prototype.name == "" {
            match &function.body {
                ConditionalExpr{ cond_expr, then_expr, else_expr } => (),
                LoopExpr{ var_name, start_expr, end_expr, step_expr, body_expr } => (),
                _ => translated_expression.push(';')
            }

            translated_expression.push_str("\n");
            translated_expressions.push_str(&translated_expression);
        }
        // Function definition
        else {
            let translated_function_definition = translate_function_definition(&function.prototype, &translated_expression);
            translated_function_definitions.push_str(&translated_function_definition);
        }
    }

    // Constructing simple Rust source code file structure
    let rust_source_code = format!("{}\nfn main() {{\n{}\n}}",
        translated_function_definitions,
        translated_expressions
    );

    Ok(rust_source_code)
}

fn translate_function_definition(prototype: &Prototype, translated_body: &String) -> String {
    let mut function_arguments_string = String::new();

    if prototype.args.len() > 0 {
        function_arguments_string = prototype.args.join(": f64, ");
        function_arguments_string.push_str(": f64");
    }

    let return_type_str = match prototype.ret {
        FunctionReturnType::Void => "()",
        FunctionReturnType::Num => "f64",
        FunctionReturnType::Str => "String"
    };

    let mut function_definition_str = format_function_definition!(prototype.name, function_arguments_string, return_type_str, translated_body);
    function_definition_str.push_str("\n");

    return function_definition_str;
}

fn translate_expression(expr: &Expression) -> String {
    let translated_expr = match expr {
        LiteralExpr(expr) => translate_literal_expression(expr),
        VariableExpr(expr) => translate_variable_expression(expr),
        BinaryExpr(operator, lhs, rhs) => translate_binary_expression(operator, lhs, rhs),
        ConditionalExpr{ cond_expr, then_expr, else_expr } => translate_conditional_expression(cond_expr, then_expr, else_expr),
        LoopExpr{ var_name, start_expr, end_expr, step_expr, body_expr } => translate_loop_expression(),
        VarInitExpr(name, value) => translate_var_init_expression(name, value),
        CallExpr(name, args) => translate_call_expression(name, args)
    };

    translated_expr
}

fn translate_literal_expression(value: &f64) -> String {
    format!("{:.*}", 2, value)
}

fn translate_variable_expression(name: &String) -> String {
    format!("{}", String::from(name.as_str()))
}

fn translate_binary_expression(operator: &String, lhs: &Expression, rhs: &Expression) -> String {
    let translated_lhs: String = translate_expression(lhs);
    let translated_rhs: String = translate_expression(rhs);

    format!("({} {} {})", translated_lhs, operator, translated_rhs)
}

fn translate_conditional_expression(if_expr: &Expression, then_expr: &Expression, else_expr: &Expression) -> String {
    let translated_if_expr = translate_expression(if_expr);
    let translated_then_expr = translate_expression(then_expr);
    let translated_else_expr = translate_expression(else_expr);

    format_conditional!(translated_if_expr, translated_then_expr, translated_else_expr)
}

fn translate_loop_expression() -> String {
    // TODO:
    format!("{}", "[loop expression]")
}

fn translate_var_init_expression(name: &String, value: &Expression) -> String {
    let translated_value: String = translate_expression(value);

    format!("let {} = {}", name, translated_value)
}

fn translate_call_expression(function_name: &String, args: &Vec<Expression>) -> String {
    let mut translated_args = Vec::new();
    for arg in args {
        let translated_arg = translate_expression(arg);
        translated_args.push(translated_arg);
    }

    format!("{}({})", function_name, translated_args.join(", "))
}
