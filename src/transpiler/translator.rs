use crate::transpiler::parser::*;

macro_rules! function_definition {
    ($name:expr, $body:expr, $args:expr) => (
        format!("fn {}({}) {{\n{:?}\n}}", $name, $args, $body);
    )
}

pub fn translate(ast: &[ASTNode]) -> Result<String, String> {
    let mut translated_expressions = String::new();
    let mut translated_function_definitions = String::new();

    for node in ast {
        let FunctionNode(function) = node;
        

        // println!("{:?}", function);


        // Not function definition
        if function.prototype.name == "" {
            let mut translated_expr_string: String = match &function.body {
                LiteralExpr(expr) => format!("{};", expr.to_string()),
                VariableExpr(expr) => format!("{};", String::from(expr.as_str())),
                BinaryExpr(operator, lhs, rhs) => translate_binary_expression(operator, lhs, rhs),
                ConditionalExpr{ cond_expr, then_expr, else_expr } => translate_conditional_expression(),
                LoopExpr{ var_name, start_expr, end_expr, step_expr, body_expr } => translate_loop_expression(),
                CallExpr(name, args) => translate_call_expression()
            };

            translated_expr_string.push_str("\n");
            translated_expressions.push_str(&translated_expr_string);
        }
        // Function definition
        else {
            let mut function_definition_string = function_definition!(function.prototype.name, function.body, function.prototype.args.join(", "));

            function_definition_string.push_str("\n");
            translated_function_definitions.push_str(&function_definition_string);
        }
    }

    // Constructing simple Rust source code file structure
    let rust_source_code = format!("{}\nfn main() {{\n{}\n}}",
        translated_function_definitions,
        translated_expressions
    );

    Ok(rust_source_code)
}

fn translate_binary_expression(operator: &String, lhs: &Expression, rhs: &Expression) -> String {
    // println!("{:?} {:?} {:?}", operator, lhs, rhs);



    format!("{};", "[binary expression]")
}

fn translate_conditional_expression() -> String {
    // TODO:
    format!("{};", "[conditional expression]")
}

fn translate_loop_expression() -> String {
    // TODO:
    format!("{};", "[loop expression]")
}

fn translate_call_expression() -> String {
    // TODO:
    format!("{};", "[call expression]")
}
