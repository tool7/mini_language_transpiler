use std::fs::File;

use crate::transpiler::parser::*;

pub fn translate(ast: &[ASTNode]) {
    let mut output_file_path = File::create("output/test1.rs");

    for node in ast {
        let FunctionNode(function) = node;
        
        println!("{:?}", function);


        // Not function definition
        if function.prototype.name == "" {
            match &function.body {
                LiteralExpr(expr) => (),
                VariableExpr(expr) => (),
                BinaryExpr(operator, lhs, rhs) => (),
                ConditionalExpr{ cond_expr, then_expr, else_expr } => (),
                LoopExpr{ var_name, start_expr, end_expr, step_expr, body_expr } => (),
                CallExpr(name, args) => ()
            }
        }
        // Function definition
        else {

        }
    }
}
