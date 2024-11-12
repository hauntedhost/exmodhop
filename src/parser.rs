use tree_sitter::{Node, Parser, Tree};
use tree_sitter_elixir;

// TODO: return Result<Tree, _>
pub fn parse_elixir(source_code: &str) -> Tree {
    let language = tree_sitter_elixir::LANGUAGE;

    let mut parser = Parser::new();
    parser
        .set_language(&language.into())
        .expect("Error creating Elixir parser");

    parser
        .parse(source_code, None)
        .expect("Error parsing source code")
}

// TODO: return Result<(), _>
pub fn collect_path_modules(
    node: Node,
    source_code: &str,
    source_pathname: &str,
    module_parts: &mut Vec<String>,
    path_modules: &mut Vec<(String, usize)>,
) {
    let mut cursor = node.walk();

    // If we find a defmodule call, collect the module name
    if node.kind() == "call" {
        let mut children = node.named_children(&mut cursor);
        if let (Some(first_child), Some(second_child)) = (children.next(), children.next()) {
            if is_defmodule(first_child, source_code) {
                let module_name = node_name(second_child, source_code);
                module_parts.push(module_name);
                let module_name = module_parts.join(".");
                let line_number = node.start_position().row + 1;
                path_modules.push((module_name, line_number));
            }
        }
    }

    // Recurse into child nodes
    for child in node.named_children(&mut cursor) {
        collect_path_modules(
            child,
            source_code,
            source_pathname,
            module_parts,
            path_modules,
        );
    }

    // Backtrack after processing this module
    if node.kind() == "call" {
        for child in node.children(&mut cursor) {
            if is_defmodule(child, source_code) {
                module_parts.pop();
                break;
            }
        }
    }
}

fn is_defmodule(node: Node, source_code: &str) -> bool {
    let node_type = node.kind();
    let node_name = node
        .utf8_text(source_code.as_bytes())
        .expect("Error getting node name");

    node_type == "identifier" && node_name == "defmodule"
}

fn node_name(node: Node, source_code: &str) -> String {
    node.utf8_text(source_code.as_bytes())
        .expect("Error getting node name")
        .to_string()
}
