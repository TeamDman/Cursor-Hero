use syn::{parse_file, visit_mut::VisitMut, Expr, File};
use quote::ToTokens;
use std::fs;

struct WindowPluginVisitor;

impl VisitMut for WindowPluginVisitor {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        // Logic to find and modify the WindowPlugin configuration
        // This is a simplified example. The actual implementation
        // would require precise matching and modification
        if let Expr::Struct(es) = i {
            if es.path.is_ident("WindowPlugin") {
                // Modify the fields here
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ast = parse_file(&fs::read_to_string("src/main.rs")?)?;

    let mut visitor = WindowPluginVisitor;
    visitor.visit_file_mut(&mut ast);

    fs::write("src/main.rs", ast.into_token_stream().to_string())?;

    Ok(())
}

