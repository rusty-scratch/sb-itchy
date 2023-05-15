#![cfg(feature = "block-definer")]

use sb_block_def_genie::*;
use std::{collections::BTreeSet, format as f};

pub struct Paths {
    pub block_field_builder: String,
    pub block_input_builder: String,
    pub block_normal_builder: String,
    pub stack_builder: String,
}

impl Paths {
    pub fn from_crate_path(crate_path: &str) -> Paths {
        Paths {
            block_field_builder: f!("{crate_path}::block::BlockFieldBuilder"),
            block_input_builder: f!("{crate_path}::block::BlockInputBuilder"),
            block_normal_builder: f!("{crate_path}::block::BlockNormalBuilder"),
            stack_builder: f!("{crate_path}::stack::StackBuilder"),
        }
    }
}

pub fn define(blocks: &Blocks, paths: &Paths) -> (String, Vec<String>) {
    let Paths { stack_builder, .. } = paths;
    let mut definitions = String::new();
    let mut messages = vec![];
    for block in &blocks.0 {
        let Block {
            identifier,
            opcode,
            doc,
            implementor_note,
            manual,
            block_type: _,
            parameters,
        } = block;
        if *manual {
            messages.push(f!("`{identifier}` requried manual implementation"));
            continue;
        }
        if let Some(implementor_note) = implementor_note {
            messages.push(f!("{identifier}: {implementor_note}"));
        }

        let identifier = dekeyword(identifier).unwrap_or(identifier);

        let (parameters_string, parameters_doc) =
            build_fn_parameter(identifier, &mut messages, parameters, paths);
        let parameters_doc = str_lines_to_doc(&parameters_doc);
        let doc = str_lines_to_doc(doc);
        let fn_body = build_fn_body(opcode, &block.block_type, parameters, paths);
        let fn_item = f!("\
            {doc}\
            {parameters_doc}\
            pub fn {identifier}({parameters_string}) -> {stack_builder} {{\n\
                {fn_body}\
            }}\n\
            \n\
        ");
        definitions.push_str(&fn_item);
    }
    (definitions, messages)
}

fn str_lines_to_doc(str: &str) -> String {
    str.lines().map(|s| f!("/// {s}\n")).collect()
}

fn build_fn_parameter(
    block_identifier: &str,
    messages: &mut Vec<String>,
    parameters: &[ParameterOrText],
    paths: &Paths,
) -> (String, String) {
    let Paths {
        block_field_builder: bfb,
        block_input_builder: bib,
        ..
    } = paths;

    let mut parameters_string = String::new();
    let mut parameters_doc = String::new();
    let parameters = parameters.iter().filter_map(|pot| match pot {
        ParameterOrText::Parameter(p) => Some(p),
        ParameterOrText::Text(_) => None,
    });
    for parameter in parameters {
        let Parameter {
            identifier,
            parameter_type,
            key: _,
            deny_strings,
        } = parameter;

        let identifier = dekeyword(identifier).unwrap_or(&identifier[..]);

        let parameter: String;
        let mut parameter_doc = None;
        match parameter_type {
            ParameterType::Field {
                possible_values,
                possible_categories,
            } => {
                parameter_doc = build_fn_parameter_doc(
                    identifier,
                    possible_values.as_ref(),
                    possible_categories.as_ref(),
                );
                parameter = f!("{identifier}: {bfb}, ");
            }
            ParameterType::Block => {
                parameter = f!("{identifier}: {bib}, ");
            }
            _ => {
                parameter = f!("{identifier}: {bib}, ");
            }
        }
        if deny_strings.is_some() {
            messages.push(f!(
                "{block_identifier}.{identifier} deny strings aren't supproted"
            ));
        }
        if let Some(doc) = parameter_doc {
            parameters_doc.push_str(&doc)
        }
        parameters_string.push_str(&parameter);
    }

    (parameters_string, parameters_doc)
}

fn build_fn_parameter_doc(
    identifier: &str,
    possible_values: Option<&BTreeSet<String>>,
    possible_categories: Option<&BTreeSet<String>>,
) -> Option<String> {
    if !(possible_values.is_some() || possible_categories.is_some()) {
        return None;
    }

    let mut s = String::new();
    s.push_str(&f!("`{identifier}` accepts:\n"));
    if let Some(possible_categories) = possible_categories {
        for category in possible_categories {
            s.push_str(&f!(" - {category}\n"));
        }
    }
    if let Some(possible_values) = possible_values {
        for value in possible_values {
            s.push_str(&f!(" - `\"{value}\"`\n"));
        }
    }
    Some(s)
}

fn build_fn_body(
    opcode: &str,
    block_type: &BlockType,
    parameters: &[ParameterOrText],
    paths: &Paths,
) -> String {
    let Paths {
        block_normal_builder,
        stack_builder,
        ..
    } = paths;
    let mut body = String::new();
    let parameters = parameters.iter().filter_map(|pot| match pot {
        ParameterOrText::Parameter(p) => Some(p),
        ParameterOrText::Text(_) => None,
    });
    let mut parameters = parameters.peekable();
    let there_is_some_parameter = parameters.peek().is_some();
    let is_menu = *block_type == BlockType::Menu;
    let need_changes = there_is_some_parameter | is_menu;
    if need_changes {
        body.push_str(&f!(
            "    let mut block_builder = {block_normal_builder}::new(\"{opcode}\");\n"
        ));
    } else {
        body.push_str(&f!(
            "    let block_builder = {block_normal_builder}::new(\"{opcode}\");\n"
        ));
    }
    if there_is_some_parameter {
        for parameter in parameters {
            let Parameter {
                identifier,
                key,
                parameter_type,
                deny_strings: _,
            } = parameter;
            match parameter_type {
                ParameterType::Field { .. } => body.push_str(&f!(
                    "    block_builder.add_field(\"{key}\", {identifier});\n"
                )),
                _ => body.push_str(&f!(
                    "    block_builder.add_input(\"{key}\", {identifier});\n"
                )),
            }
        }
    }
    if is_menu {
        body.push_str("    block_builder.set_shadow(true);\n")
    }
    body.push_str(&f!("    {stack_builder}::start(block_builder)\n"));
    body
}

fn dekeyword(token: &str) -> Option<&str> {
    let new = match token {
        "if" => "if_",
        "as" => "as_",
        "break" => "break_",
        "const" => "const_",
        "continue" => "continue_",
        "crate" => "crate_",
        "else" => "else_",
        "enum" => "enum_",
        "extern" => "extern_",
        "false" => "false_",
        "fn" => "fn_",
        "for" => "for_",
        "impl" => "impl_",
        "in" => "in_",
        "let" => "let_",
        "loop" => "loop_",
        "match" => "match_",
        "mod" => "mod_",
        "move" => "move_",
        "mut" => "mut_",
        "pub" => "pub_",
        "ref" => "ref_",
        "return" => "return_",
        "self" => "self_",
        "Self" => "Self_",
        "static" => "static_",
        "struct" => "struct_",
        "super" => "super_",
        "trait" => "trait_",
        "true" => "true_",
        "type" => "type_",
        "unsafe" => "unsafe_",
        "use" => "use_",
        "where" => "where_",
        "while" => "while_",
        "async" => "async_",
        "await" => "await_",
        "dyn" => "dyn_",
        "abstract" => "abstract_",
        "become" => "become_",
        "box" => "box_",
        "do" => "do_",
        "final" => "final_",
        "macro" => "macro_",
        "override" => "override_",
        "priv" => "priv_",
        "typeof" => "typeof_",
        "unsized" => "unsized_",
        "virtual" => "virtual_",
        "yield" => "yield_",
        "try" => "try_",
        _ => return None,
    };
    Some(new)
}
