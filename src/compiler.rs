#[allow(unused)]
use crate::utils::{
    choose_compiler, cli_info, debug, djb2_hash, is_gcc_available, levenshtein_distance,
    syntax_error, SyntaxErrorType,
};

use std::{
    fs::{exists, read_to_string, remove_file, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    process::{exit, Command},
    str::from_utf8,
};

#[derive(PartialEq, Clone, Debug)]
struct FunctionDeclaration {
    name: String,
    n_args: usize,
}

fn fn_decl(name: &str, n_args: usize) -> FunctionDeclaration {
    FunctionDeclaration {
        name: name.to_string(),
        n_args,
    }
}

fn handle_func_call(
    compiled: &mut String,
    tokens: &[&str],
    declared_functions: &Vec<FunctionDeclaration>,
    include_std: bool,
    std_lib_functions: &[FunctionDeclaration],
    line_nr: usize,
    line: &str,
) {
    let fn_name = tokens[1];
    let tokens_len = tokens.len();

    match fn_name {
        "print" => {
            let tmp = if tokens_len < 3 {
                String::new()
            } else {
                // FIXME: not till the end but till the end of the string literal (so it will
                // be possible to have multiple string literals on a single line)
                tokens[2..tokens_len].join(" ")
            };
            let mut to_print = String::new();
            for i in 0..tmp.len() {
                let mut tmp_chars = tmp.chars();
                if let Some(c) = tmp_chars.nth(i) {
                    if c == '_' {
                        if i != 0 && tmp_chars.nth(i - 1) == Some('\\') {
                            to_print.push('_');
                        } else {
                            to_print.push(' ');
                        }
                    } else {
                        to_print.push(c);
                    }
                }
            }
            if to_print.starts_with('\"') {
                compiled.push_str(&format!("printf({to_print});\n"));
            } else if to_print.starts_with('%') {
                let format_string = tokens[2];
                let to_print = tokens[3..tokens_len].join(" ");
                compiled.push_str(&format!("printf(\"{format_string}\", {to_print});\n"));
            } else {
                compiled.push_str(&format!("printf(\"{to_print}\");\n"));
            }
        }
        "eprint" => {
            let tmp = if tokens_len < 3 {
                String::new()
            } else {
                // FIXME: not till the end but till the end of the string literal (so it will
                // be possible to have multiple string literals on a single line)
                tokens[2..tokens_len].join(" ")
            };
            let mut to_print = String::new();
            for i in 0..tmp.len() {
                let mut tmp_chars = tmp.chars();
                if let Some(c) = tmp_chars.nth(i) {
                    if c == '_' {
                        if i != 0 && tmp_chars.nth(i - 1) == Some('\\') {
                            to_print.push('_');
                        } else {
                            to_print.push(' ');
                        }
                    } else {
                        to_print.push(c);
                    }
                }
            }
            if to_print.starts_with('\"') {
                compiled.push_str(&format!("fprintf(stderr, {to_print});\n"));
            } else if to_print.starts_with('%') {
                let format_string = tokens[2];
                let to_print = tokens[3..tokens_len].join(" ");
                compiled.push_str(&format!(
                    "fprintf(stderr, \"{format_string}\", {to_print});\n"
                ));
            } else {
                compiled.push_str(&format!("fprintf(stderr, \"{to_print}\");\n"));
            }
        }
        "println" => {
            let tmp = if tokens_len < 3 {
                String::new()
            } else {
                // FIXME: not till the end but till the end of the string literal (so it will
                // be possible to have multiple string literals on a single line)
                tokens[2..tokens_len].join(" ")
            };
            let mut to_print = String::new();
            for i in 0..tmp.len() {
                let mut tmp_chars = tmp.chars();
                if let Some(c) = tmp_chars.nth(i) {
                    if c == '_' {
                        if i != 0 && tmp_chars.nth(i - 1) == Some('\\') {
                            to_print.push('_');
                        } else {
                            to_print.push(' ');
                        }
                    } else {
                        to_print.push(c);
                    }
                }
            }

            if to_print.starts_with('\"') {
                to_print.pop();
                compiled.push_str(&format!("printf({to_print}\\n\");\n"));
            } else if to_print.starts_with('%') {
                let format_string = tokens[2];
                let to_print = tokens[3..tokens_len].join(" ");
                compiled.push_str(&format!("printf(\"{format_string}\\n\", {to_print});\n"));
            } else {
                compiled.push_str(&format!("printf(\"{to_print}\\n\");\n"));
            }
        }
        "eprintln" => {
            let tmp = if tokens_len < 3 {
                String::new()
            } else {
                // FIXME: not till the end but till the end of the string literal (so it will
                // be possible to have multiple string literals on a single line)
                tokens[2..tokens_len].join(" ")
            };
            let mut to_print = String::new();
            for i in 0..tmp.len() {
                let mut tmp_chars = tmp.chars();
                if let Some(c) = tmp_chars.nth(i) {
                    if c == '_' {
                        if i != 0 && tmp_chars.nth(i - 1) == Some('\\') {
                            to_print.push('_');
                        } else {
                            to_print.push(' ');
                        }
                    } else {
                        to_print.push(c);
                    }
                }
            }

            if to_print.starts_with('\"') {
                to_print.pop();
                compiled.push_str(&format!("fprintf(stderr, {to_print}\\n\");\n"));
            } else if to_print.starts_with('%') {
                let format_string = tokens[2];
                let to_print = tokens[3..tokens_len].join(" ");
                compiled.push_str(&format!(
                    "fprintf(stderr, \"{format_string}\\n\", {to_print});\n"
                ));
            } else {
                compiled.push_str(&format!("fprintf(stderr, \"{to_print}\\n\");\n"));
            }
        }
        _ => {
            let decl_func = declared_functions
                .iter()
                .find(|&f| f.name == fn_name)
                .unwrap_or_else(|| {
                    if include_std {
                        let mut most_similar_fn = "";
                        for func in declared_functions {
                            let ldist = levenshtein_distance(&func.name, fn_name);
                            if ldist < 3 {
                                if !most_similar_fn.is_empty() {
                                    let ldist_most_similar =
                                        levenshtein_distance(&func.name, most_similar_fn);
                                    if ldist < ldist_most_similar {
                                        most_similar_fn = &func.name;
                                    }
                                } else {
                                    most_similar_fn = &func.name;
                                }
                            }
                        }
                        syntax_error(
                            line_nr,
                            line,
                            SyntaxErrorType::FunctionDoesntExist,
                            if most_similar_fn.is_empty() {
                                None
                            } else {
                                Some(most_similar_fn)
                            },
                        );
                    } else {
                        if let Some(std_func) = std_lib_functions.iter().find(|f| f.name == fn_name)
                        {
                            syntax_error(
                                line_nr,
                                line,
                                SyntaxErrorType::StdLibFuncNoStd,
                                Some(&std_func.name),
                            );
                        }
                        syntax_error(line_nr, line, SyntaxErrorType::FunctionDoesntExist, None);
                    }
                    unreachable!();
                });

            let n_args = tokens_len - 2;
            if n_args != decl_func.n_args {
                syntax_error(line_nr, line, SyntaxErrorType::WrongNumOfArgs, None);
            }

            if tokens_len > 2 {
                // has arguments
                compiled.push_str(&format!(
                    "{fn_name}({});\n",
                    tokens[2..tokens_len].join(", ")
                ));
            } else {
                // void function
                compiled.push_str(&format!("{fn_name}();\n"));
            }
        }
    }
}

fn handle_imports(source_code: &str) -> String {
    let mut code_lines: Vec<String> = source_code.lines().map(|x| x.to_owned()).collect();
    for (i, line) in code_lines.iter_mut().enumerate() {
        if let Some(first) = line.split_whitespace().next() {
            if first == "import" {
                let path = line.split_whitespace().nth(1).unwrap();
                *line = handle_imports(&read_to_string(path).unwrap_or_else(|_| {
                    syntax_error(i, line, SyntaxErrorType::ImportNotFound, None);
                    String::new()
                }));
            }
        }
    }
    code_lines.join("\n")
}

pub fn compile(source_code: &str) -> String {
    let mut compiled = String::new();
    let mut include_std = true;

    let mut std_lib_functions = vec![
        fn_decl("print", 0),
        fn_decl("println", 0),
        fn_decl("helloworld", 0),
        fn_decl("initVecString", 1),
        fn_decl("newVecString", 0),
        fn_decl("resizeVecString", 1),
        fn_decl("freeVecString", 1),
        fn_decl("pushVecString", 2),
        fn_decl("getVecString", 2),
        fn_decl("stringSplit", 2),
        fn_decl("stringSplitWhitespace", 1),
        fn_decl("stringTrim", 1),
        fn_decl("stringEmpty", 1),
        fn_decl("readToString", 1),
        fn_decl("readToLines", 1),
        fn_decl("readToLinesNonEmpty", 1),
        fn_decl("stringCmp", 2),
        fn_decl("strlen", 1),
        fn_decl("splitToLines", 1),
        fn_decl("joinVecString", 2),
        fn_decl("modVecString", 3),
        fn_decl("djb2Hash", 1),
        fn_decl("getArgsVec", 2),
        fn_decl("containsVecString", 2),
        fn_decl("findVecString", 2),
    ];

    let mut declared_functions: Vec<FunctionDeclaration> = Vec::new();

    for (line_nr, line) in source_code.lines().enumerate() {
        if line_nr == 0 {
            if line.split_whitespace().collect::<String>() == "//nostd" {
                include_std = false;
            } else {
                declared_functions.append(&mut std_lib_functions);
            }
        }
        if line.trim().is_empty() || line.trim().starts_with("//") {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let tokens_len = tokens.len();

        // debug(&tokens);

        match tokens[0] {
            "fn" => {
                if tokens.last().unwrap() != &"{" {
                    syntax_error(line_nr, line, SyntaxErrorType::MissingBracket, None);
                }
                if tokens_len < 3 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                let mut return_type = "void";
                if tokens[tokens_len - 3] == ">" {
                    return_type = tokens[tokens_len - 2];
                }
                // debug(&format!("RETURN TYPE: {return_type}"));

                let fn_name = tokens[1];

                if tokens_len > if tokens[tokens_len - 3] == ">" { 5 } else { 3 } {
                    // has arguments
                    let argument_tokens =
                        &tokens[2..tokens_len - if tokens[tokens_len - 3] == ">" { 3 } else { 1 }];
                    if argument_tokens.len() % 2 != 0 {
                        syntax_error(
                            line_nr,
                            line,
                            SyntaxErrorType::InvalidFunctionArguments,
                            None,
                        );
                    }
                    let chunks: Vec<_> = argument_tokens.chunks(2).collect();
                    let arguments: Vec<_> =
                        chunks.into_iter().map(|chunk| chunk.join(" ")).collect();
                    compiled.push_str(&format!(
                        "{return_type} {fn_name}({}) {{\n",
                        arguments.join(", ")
                    ));
                    declared_functions.push(fn_decl(fn_name, argument_tokens.len() / 2));
                } else {
                    compiled.push_str(&format!("{return_type} {fn_name}() {{\n"));
                    declared_functions.push(fn_decl(fn_name, 0));
                }
            }
            "var" => {
                if tokens_len < 3 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                let var_type = tokens[1];
                let var_name = tokens[2];

                if tokens_len > 3 {
                    if tokens[3] == "()" {
                        // handle function call
                        compiled.push_str(&format!("{var_type} {var_name} = "));
                        let tokens = &tokens[3..];
                        handle_func_call(
                            &mut compiled,
                            tokens,
                            &declared_functions,
                            include_std,
                            &std_lib_functions,
                            line_nr,
                            line,
                        );
                    } else {
                        // varriable get's assigned a value
                        compiled.push_str(&format!("{var_type} {var_name} = {};\n", tokens[3]));
                    }
                } else {
                    // declaration alone
                    compiled.push_str(&format!("{var_type} {var_name};\n"));
                }
            }
            "loop" => {
                if tokens.last().unwrap() != &"{" {
                    syntax_error(line_nr, line, SyntaxErrorType::MissingBracket, None);
                }
                if tokens_len < 2 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                if tokens_len == 2 {
                    compiled.push_str("while (1) {\n");
                } else {
                    compiled.push_str(&format!(
                        "while ({}) {{\n",
                        tokens[1..tokens_len - 1].join(" ")
                    ));
                }
            }
            "()" => {
                if tokens_len < 2 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                handle_func_call(
                    &mut compiled,
                    &tokens,
                    &declared_functions,
                    include_std,
                    &std_lib_functions,
                    line_nr,
                    line,
                );
            }
            "=" => {
                if tokens_len < 3 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                let var_name = tokens[1];

                if tokens[2] == "()" {
                    // handle function call
                    compiled.push_str(&format!("{var_name} = "));
                    let tokens = &tokens[2..];
                    handle_func_call(
                        &mut compiled,
                        tokens,
                        &declared_functions,
                        include_std,
                        &std_lib_functions,
                        line_nr,
                        line,
                    );
                } else {
                    compiled.push_str(&format!(
                        "{var_name} = {};\n",
                        tokens[2..tokens_len].join(" ")
                    ));
                }
            }
            "}" => compiled.push_str("}\n"),
            "ret" => {
                compiled.push_str(&format!(
                    "return {};\n",
                    line.trim().replace("ret", "").trim()
                ));
            }
            "continue" => {
                compiled.push_str("continue;\n");
            }
            "if" => {
                if tokens_len < 3 || tokens.last().unwrap() != &"{" {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                compiled.push_str(&format!(
                    "if ({}) {{\n",
                    tokens[1..tokens_len - 1].join(" ")
                ));
            }
            "elif" => {
                if tokens_len < 3 || tokens.last().unwrap() != &"{" {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                compiled.push_str(&format!(
                    "else if ({}) {{\n",
                    tokens[1..tokens_len - 1].join(" ")
                ));
            }
            "else" => {
                if tokens_len != 2 || tokens.last().unwrap() != &"{" {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                compiled.push_str("else {\n");
            }
            "C" => {
                if tokens_len == 1 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                let line = line
                    .trim()
                    .chars()
                    .skip(1)
                    .collect::<String>()
                    .trim()
                    .to_owned();
                compiled.push_str(&(line + "\n"));
            }
            "export" => {
                if tokens_len < 3 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                declared_functions.push(FunctionDeclaration {
                    name: tokens[1].to_string(),
                    n_args: tokens[2].parse().unwrap_or_else(|_| {
                        syntax_error(line_nr, line, SyntaxErrorType::General, None);
                        unreachable!();
                    }),
                });
            }
            "++" => {
                if tokens_len < 2 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                let var_name = tokens[1];
                compiled.push_str(&format!("{var_name} = {var_name} + 1;\n"));
            }
            "--" => {
                if tokens_len < 2 {
                    syntax_error(line_nr, line, SyntaxErrorType::General, None);
                }

                let var_name = tokens[1];
                compiled.push_str(&format!("{var_name} = {var_name} - 1;\n"));
            }
            _ => {
                syntax_error(line_nr, line, SyntaxErrorType::UnknownKeyword, None);
            }
        }
    }

    let stdlib = include_str!("stdlib.c").to_string();

    if *crate::utils::DEBUG.lock().unwrap() {
        println!("{}{compiled}", if include_std { &stdlib } else { "" });
    }

    if include_std {
        stdlib + &compiled
    } else {
        compiled
    }
}

pub fn compile_to_file(source_file: String, output_file: String, release_build: bool) {
    let source_code =
        handle_imports(&read_to_string(source_file).expect("Could not read source code file"));
    // debug(&source_code);
    let source_code_hash = &djb2_hash(&source_code).to_be_bytes();

    // check if source code hash is different than the current source code
    if exists(&output_file).unwrap() {
        let mut f = File::open(&output_file).unwrap();
        let mut buf = vec![0u8; 14];
        f.read_exact(&mut buf).unwrap();
        let bin_hash = &buf[10..14];
        if bin_hash == source_code_hash {
            cli_info("No change detected. No recompilation needed.");
            return;
        }
    }

    let compiler = choose_compiler();
    let gcc_available = is_gcc_available();

    // TODO: release builds with clang (fallback)
    if release_build && !gcc_available {
        eprintln!("ERROR: Building in release mode but GCC not available");
        exit(103);
    }

    let compiled = compile(&source_code);
    let compiler_ir_fname = "compiler_ir.c";

    let mut compiled_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(compiler_ir_fname)
        .expect("Failed to open compiler IR file");

    compiled_file
        .write_all(compiled.as_bytes())
        .expect("Failed to write compiled code to IR file");

    // actually compile using tcc or gcc or clang
    if release_build {
        // TODO: also allow release builds with clang (fallback) (update help screen)
        let output = Command::new("gcc")
            .args(["-O3", "-o", &output_file, compiler_ir_fname])
            .output()
            .expect("GCC execution failed");
        if !output.status.success() {
            println!("{}", from_utf8(&output.stderr).unwrap());
        }
    } else {
        match compiler {
            "tcc" => {
                let output = Command::new("tcc")
                    .args(["-o", &output_file, compiler_ir_fname])
                    .output()
                    .expect("TCC execution failed");
                if !output.status.success() {
                    println!("{}", from_utf8(&output.stderr).unwrap());
                }
            }
            "gcc" => {
                let output = Command::new("gcc")
                    .args(["-o", &output_file, compiler_ir_fname])
                    .output()
                    .expect("GCC execution failed");
                if !output.status.success() {
                    println!("{}", from_utf8(&output.stderr).unwrap());
                }
            }
            "clang" => {
                let output = Command::new("clang")
                    .args(["-o", &output_file, compiler_ir_fname])
                    .output()
                    .expect("Clang execution failed");
                if !output.status.success() {
                    println!("{}", from_utf8(&output.stderr).unwrap());
                }
            }
            _ => (),
        }
    }

    // remove the temporary compiler IR file
    remove_file(compiler_ir_fname).expect("Failed to remove compiler IR file");

    // write source code hash to out binary elf header
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&output_file)
        .unwrap();
    f.seek(SeekFrom::Start(10)).unwrap();
    f.write_all(source_code_hash).unwrap();
}
