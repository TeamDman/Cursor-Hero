use syn::{parse_file, visit_mut::VisitMut, Expr, File, visit::Visit, ItemFn};
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


// Define a visitor that will find function items.
struct FnVisitor;

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        println!("Found function: {}", i.sig.ident);
        // Continue traversing the AST as normal
        syn::visit::visit_item_fn(self, i);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ast = parse_file(&fs::read_to_string("src/main.rs")?)?;

    let mut visitor = WindowPluginVisitor;
    visitor.visit_file_mut(&mut ast);


    
    // Create a visitor
    let mut visitor = FnVisitor;

    // Visit the syntax tree
    visitor.visit_file(&ast);

    
    // fs::write("src/main.rs", ast.into_token_stream().to_string())?;

    Ok(())
}

