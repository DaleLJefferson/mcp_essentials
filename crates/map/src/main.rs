use tree_sitter::{Node, Parser};

fn main() {
    // Example input from the query
    let source_code = r#"pub struct MyStruct {
         pub public_field: i32,
         private_field: i32,
         pub pub_field: String
     }"#;

    // Initialize the parser
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("Error loading Rust grammar");

    // Parse the source code into an AST
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    // Vector to collect output lines
    let mut output = Vec::new();

    // Traverse top-level items
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        if is_public(&child, source_code) && child.kind() == "struct_item" {
            let struct_def = process_struct(&child, source_code);
            output.push(struct_def);
        }
    }

    // Print the output
    println!("{}", output.join("\n"));
}

// Check if a node is public
fn is_public(node: &Node, source: &str) -> bool {
    node.children(&mut node.walk()).any(|child| {
        child.kind() == "visibility_modifier"
            && child.utf8_text(source.as_bytes()).unwrap() == "pub"
    })
}

// Process a public struct and return its external interface
fn process_struct(node: &Node, source: &str) -> String {
    // Extract struct name
    let name_node = node.child_by_field_name("name").unwrap();
    let name = name_node.utf8_text(source.as_bytes()).unwrap();

    // Get the field declaration list if it exists (it's called "body" in the AST)
    let field_list_node = node.child_by_field_name("body");

    // If there's no field list, return an empty struct
    if field_list_node.is_none() {
        return format!("pub struct {} {{}}", name);
    }

    let field_list_node = field_list_node.unwrap();

    // Collect public fields
    let mut public_fields = Vec::new();
    let mut cursor = field_list_node.walk();
    for child in field_list_node.children(&mut cursor) {
        if child.kind() == "field_declaration" && is_public(&child, source) {
            let field_text = child.utf8_text(source.as_bytes()).unwrap();
            // Preserve original indentation by extracting the full text
            public_fields.push(format!("     {}", field_text));
        }
    }

    // Construct the struct definition
    if public_fields.is_empty() {
        format!("pub struct {} {{}}", name)
    } else {
        format!("pub struct {} {{\n{}\n}}", name, public_fields.join(",\n"))
    }
}
