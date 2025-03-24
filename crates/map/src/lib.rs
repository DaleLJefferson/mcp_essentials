use tree_sitter::{Node, Parser};

#[derive(Debug, PartialEq)]
enum ItemKind {
    Struct,
    Enum,
    Const,
    Impl,
    Other(String),
}

impl ItemKind {
    fn from_node_kind(kind: &str) -> Self {
        match kind {
            "struct_item" => ItemKind::Struct,
            "enum_item" => ItemKind::Enum,
            "const_item" => ItemKind::Const,
            "impl_item" => ItemKind::Impl,
            k => ItemKind::Other(k.to_string()),
        }
    }
}

pub fn map(source_code: &str) -> String {
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

    // Map to store impl blocks by type name
    let mut impl_blocks = std::collections::HashMap::new();

    // First pass: collect all impl blocks for public structs
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        if child.kind() == "impl_item" {
            if let Some(impl_info) = process_impl(&child, source_code) {
                impl_blocks
                    .entry(impl_info.0)
                    .or_insert_with(Vec::new)
                    .push(impl_info.1);
            }
        }
    }

    // Second pass: traverse top-level items
    let mut cursor = root_node.walk();
    for child in root_node.children(&mut cursor) {
        if is_public(&child, source_code) {
            match ItemKind::from_node_kind(child.kind()) {
                ItemKind::Struct => {
                    let mut struct_output = process_struct(&child, source_code);

                    // Get struct name and add its impl blocks if any
                    if let Some(name_node) = child.child_by_field_name("name") {
                        let name = name_node.utf8_text(source_code.as_bytes()).unwrap();
                        if let Some(impls) = impl_blocks.get(name) {
                            for impl_block in impls {
                                if !impl_block.is_empty() {
                                    struct_output = format!("{}\n\n{}", struct_output, impl_block);
                                }
                            }
                        }
                    }

                    output.push(struct_output);
                }
                ItemKind::Enum => output.push(process_enum(&child, source_code)),
                ItemKind::Const => output.push(process_const(&child, source_code)),
                ItemKind::Impl => {} // Handled in the first pass
                ItemKind::Other(k) => panic!("Unsupported item kind: {}", k),
            }
        }
    }

    // Print the output
    output.join("\n\n")
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

    // If there's no field list, check if this is a unit struct with a semicolon
    if field_list_node.is_none() {
        let struct_text = node.utf8_text(source.as_bytes()).unwrap();
        if struct_text.contains(";") {
            return format!("pub struct {};", name);
        } else {
            return format!("pub struct {} {{}}", name);
        }
    }

    let field_list_node = field_list_node.unwrap();

    // Collect public fields
    let mut public_fields = Vec::new();
    let mut cursor = field_list_node.walk();

    // Track if we need to add a comma (for all fields except the last one)
    let mut field_nodes = Vec::new();
    for child in field_list_node.children(&mut cursor) {
        if child.kind() == "field_declaration" && is_public(&child, source) {
            field_nodes.push(child);
        }
    }

    for (index, child) in field_nodes.iter().enumerate() {
        let field_text = child.utf8_text(source.as_bytes()).unwrap();
        let field_text = field_text.trim();

        // Add a comma if it's not the last field or if the original field has a comma
        let with_comma = if index < field_nodes.len() - 1 || field_text.ends_with(',') {
            format!("{},", field_text)
        } else {
            field_text.to_string()
        };

        public_fields.push(format!("    {}", with_comma));
    }

    // Construct the struct definition
    if public_fields.is_empty() {
        format!("pub struct {} {{}}", name)
    } else {
        format!("pub struct {} {{\n{}\n}}", name, public_fields.join("\n"))
    }
}

// Process a public enum and return its external interface
fn process_enum(node: &Node, source: &str) -> String {
    // Extract enum name
    let name_node = node.child_by_field_name("name").unwrap();
    let name = name_node.utf8_text(source.as_bytes()).unwrap();

    // Get the variant list if it exists (it's called "body" in the AST)
    let variant_list_node = node.child_by_field_name("body");

    // If there's no variant list, return an empty enum
    if variant_list_node.is_none() {
        return format!("pub enum {} {{}}", name);
    }

    let variant_list_node = variant_list_node.unwrap();

    // Collect variants
    let mut variants = Vec::new();
    let mut cursor = variant_list_node.walk();

    // Track if we need to add a comma (for all variants except the last one)
    let mut variant_nodes = Vec::new();
    for child in variant_list_node.children(&mut cursor) {
        if child.kind() == "enum_variant" {
            variant_nodes.push(child);
        }
    }

    for (index, child) in variant_nodes.iter().enumerate() {
        // Get the name of the variant
        let name_node = child.child_by_field_name("name").unwrap();
        let name = name_node.utf8_text(source.as_bytes()).unwrap();

        // Add a comma if it's not the last variant
        let with_comma = if index < variant_nodes.len() - 1 {
            format!("{},", name)
        } else {
            name.to_string()
        };

        variants.push(format!("    {}", with_comma));
    }

    // Construct the enum definition
    if variants.is_empty() {
        format!("pub enum {} {{}}", name)
    } else {
        format!("pub enum {} {{\n{}\n}}", name, variants.join("\n"))
    }
}

// Process a public constant and return its definition
fn process_const(node: &Node, source: &str) -> String {
    // Extract the entire constant declaration
    let const_text = node.utf8_text(source.as_bytes()).unwrap();

    // Return the constant declaration as is
    const_text.to_string()
}

// Process an impl block and extract public methods
fn process_impl(node: &Node, source: &str) -> Option<(String, String)> {
    // Extract the type name this impl is for
    let type_node = node.child_by_field_name("type")?;
    let type_name = type_node.utf8_text(source.as_bytes()).unwrap();

    // Get the implementation body
    let body_node = node.child_by_field_name("body")?;

    // Collect public methods
    let mut public_methods = Vec::new();
    let mut cursor = body_node.walk();

    for child in body_node.children(&mut cursor) {
        if child.kind() == "function_item" && is_public(&child, source) {
            // Get the method signature
            let name_node = child.child_by_field_name("name")?;
            let name = name_node.utf8_text(source.as_bytes()).unwrap();

            // Get the parameters
            let mut params = Vec::new();
            let parameters_node = child.child_by_field_name("parameters")?;
            let mut param_cursor = parameters_node.walk();

            for param in parameters_node.children(&mut param_cursor) {
                if param.kind() == "parameter" {
                    let param_text = param.utf8_text(source.as_bytes()).unwrap();
                    params.push(param_text.to_string());
                }
            }

            // Get the return type if any
            let mut return_type = String::new();
            if let Some(return_node) = child.child_by_field_name("return_type") {
                return_type = format!(" -> {}", return_node.utf8_text(source.as_bytes()).unwrap());
            }

            // Construct the method signature
            let method_sig = format!("    pub fn {}({}){};", name, params.join(", "), return_type);

            public_methods.push(method_sig);
        }
    }

    // If no public methods, return None
    if public_methods.is_empty() {
        return None;
    }

    // Create the impl block
    let impl_block = format!("impl {} {{\n{}\n}}", type_name, public_methods.join("\n"));

    Some((type_name.to_string(), impl_block))
}
