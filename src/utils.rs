use std::{
    arch::asm,
    fmt::Debug,
    process::{exit, Command},
    sync::Mutex,
};

pub static DEBUG: Mutex<bool> = Mutex::new(false);

pub fn debug(msg: impl Debug) {
    if *DEBUG.lock().unwrap() {
        dbg!(msg);
    }
}

pub struct Color;

#[allow(unused)]
impl Color {
    pub const BLACK: &'static str = "\x1b[30m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const GOLD: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const PINK: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const GRAY: &'static str = "\x1b[90m";
    pub const LIGHT_RED: &'static str = "\x1b[91m";
    pub const LIGHT_GREEN: &'static str = "\x1b[92m";
    pub const YELLOW: &'static str = "\x1b[93m";
    pub const PURPLE: &'static str = "\x1b[94m";
    pub const LIGHT_PINK: &'static str = "\x1b[95m";
    pub const LIGHT_BLUE: &'static str = "\x1b[96m";
    pub const WHITE: &'static str = "\x1b[97m";
    pub const RESET: &'static str = "\x1b[00m";
    pub const BOLD: &'static str = "\x1b[1m";
    pub const FAINT: &'static str = "\x1b[2m";
    pub const ITALIC: &'static str = "\x1b[3m";
    pub const UNDERLINE: &'static str = "\x1b[4m";
    pub const BLINK: &'static str = "\x1b[5m";
    pub const INVERT: &'static str = "\x1b[7m";
    pub const STRIKE: &'static str = "\x1b[9m";
}

fn compile_error(msg: Vec<String>) {
    for (i, line) in msg.iter().enumerate() {
        if i == 0 {
            eprintln!("{}╭{} {line}", Color::BLUE, Color::RESET);
        } else {
            eprintln!("{}│{} {line}", Color::BLUE, Color::RESET);
        }
    }
    // FIXME: don't crash on first error occurance. show all errors, then quit
    exit(101);
}

pub enum SyntaxErrorType {
    General,
    MissingBracket,
    UnknownKeyword,
    FunctionDoesntExist,
    InvalidFunctionArguments,
    WrongNumOfArgs,
    StdLibFuncNoStd,
}

pub fn syntax_error(
    line_nr: usize,
    line: &str,
    error_type: SyntaxErrorType,
    additional: Option<&str>,
) {
    let mut msgs: Vec<String> = Vec::new();

    let mut syntax_error_line = |msg: &str| {
        msgs.push(format!(
            "[{}!{}] Syntax error on line {} - {msg}:",
            Color::RED,
            Color::RESET,
            line_nr + 1,
        ));
    };

    match error_type {
        SyntaxErrorType::UnknownKeyword => {
            syntax_error_line(&format!(
                "Unknown keyword \"{}\"",
                line.split_whitespace().next().unwrap()
            ));
        }
        SyntaxErrorType::FunctionDoesntExist | SyntaxErrorType::StdLibFuncNoStd => {
            // HACK so this function shows correct function name in the error message (when
            // declaring or updating a variable)
            match line.split_whitespace().next().unwrap() {
                "var" => {
                    syntax_error_line(&format!(
                        "Function doesn't exist \"{}\"",
                        line.split_whitespace().nth(4).unwrap()
                    ));
                }
                "=" => {
                    syntax_error_line(&format!(
                        "Function doesn't exist \"{}\"",
                        line.split_whitespace().nth(3).unwrap()
                    ));
                }
                _ => {
                    syntax_error_line(&format!(
                        "Function doesn't exist \"{}\"",
                        line.split_whitespace().nth(1).unwrap()
                    ));
                }
            }
        }
        SyntaxErrorType::InvalidFunctionArguments => {
            syntax_error_line("Invalid function arguments");
        }
        SyntaxErrorType::WrongNumOfArgs => {
            syntax_error_line("Wrong ammount of arguments");
        }
        _ => {
            msgs.push(format!(
                "[{}!{}] Syntax error on line {}:",
                Color::RED,
                Color::RESET,
                line_nr + 1
            ));
        }
    }
    msgs.push(format!(
        " {}=> {}{}:{} {line}{}",
        Color::GREEN,
        Color::RESET,
        line_nr + 1,
        Color::LIGHT_BLUE,
        Color::RESET
    ));
    match error_type {
        SyntaxErrorType::MissingBracket => {
            msgs.push("".to_string());
            msgs.push(format!(
                "[^-^] {}HELP: You are missing a bracket:{}",
                Color::BLUE,
                Color::RESET
            ));
            msgs.push(format!(
                "{}   +>{} {}: {}{line} {}{{{}",
                Color::BLUE,
                Color::RESET,
                line_nr + 1,
                Color::LIGHT_BLUE,
                Color::GOLD,
                Color::RESET
            ));
        }
        SyntaxErrorType::FunctionDoesntExist => {
            if let Some(fn_name) = additional {
                msgs.push("".to_string());
                msgs.push(format!(
                    "[^-^] {}HELP: Similar function exists: {fn_name}{}",
                    Color::BLUE,
                    Color::RESET
                ));
            }
        }
        SyntaxErrorType::StdLibFuncNoStd => {
            if let Some(fn_name) = additional {
                msgs.push("".to_string());
                msgs.push(format!(
                    "[^-^] {}HELP: Function \"{fn_name}\" exists in the standard library which is not included{}",
                    Color::BLUE,
                    Color::RESET
                ));
            }
        }
        _ => (),
    }

    compile_error(msgs);
}

pub fn cli_info(msg: &str) {
    println!("[{}o7{}] {msg}", Color::GREEN, Color::RESET);
}

pub fn is_tcc_available() -> bool {
    Command::new("tcc").output().is_ok()
}

pub fn is_gcc_available() -> bool {
    Command::new("gcc").arg("--version").output().is_ok()
}

pub fn is_clang_available() -> bool {
    Command::new("clang").arg("--version").output().is_ok()
}

static AVAILABLE_COMPILER: Mutex<&str> = Mutex::new("");

pub fn choose_compiler() -> &'static str {
    let mut avail_comp = AVAILABLE_COMPILER.lock().unwrap();
    if avail_comp.is_empty() {
        let compiler;
        if is_tcc_available() {
            compiler = "tcc";
        } else if is_gcc_available() {
            compiler = "gcc";
        } else if is_clang_available() {
            compiler = "clang";
        } else {
            eprintln!("ERROR: No suitable compiler found");
            exit(102);
        }

        *avail_comp = compiler;

        compiler
    } else {
        &avail_comp
    }
}

pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_len = a.chars().count();
    let b_len = b.chars().count();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    for (i, item) in matrix.iter_mut().enumerate().take(a_len + 1) {
        item[0] = i;
    }

    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                0
            } else {
                1
            };

            matrix[i][j] = *[
                matrix[i - 1][j] + 1,
                matrix[i][j - 1] + 1,
                matrix[i - 1][j - 1] + cost,
            ]
            .iter()
            .min()
            .unwrap();
        }
    }

    matrix[a_len][b_len]
}

#[allow(unused)]
pub fn rand_num() -> usize {
    let mut out: usize = 0;
    unsafe {
        asm!("rdrand {out}", out = inout(reg) out);
    }
    out
}
