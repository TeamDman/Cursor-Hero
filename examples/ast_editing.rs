use quote::ToTokens;
use std::fs;
use syn::parse_file;
use syn::visit::Visit;
use syn::visit_mut::VisitMut;
use syn::Expr;
use syn::File;
use syn::ItemFn;

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

struct FnVisitor;

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_field_value(&mut self, i: &'ast syn::FieldValue) {
        if i.member == syn::Member::Named(syn::Ident::new("primary_window", proc_macro2::Span::call_site())) {
            println!("Found primary_window");
            // You can further inspect the fields of the Window struct here
        }
        syn::visit::visit_field_value(self, i);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ast = parse_file(&fs::read_to_string("src/main.rs")?)?;

    // let mut visitor = WindowPluginVisitor;
    // visitor.visit_file_mut(&mut ast);

    // Create a visitor
    let mut visitor = FnVisitor;

    // Visit the syntax tree
    visitor.visit_file(&ast);

    // fs::write("src/main.rs", ast.into_token_stream().to_string())?;

    Ok(())
}
