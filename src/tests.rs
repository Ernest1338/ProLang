use crate::{
    compiler::compile,
    utils::{choose_compiler, rand_num},
};
use std::{
    fs::{remove_file, OpenOptions},
    io::Write,
    process::Command,
};

fn test_code(code: &str) -> bool {
    let compiler = choose_compiler();

    let test_id = rand_num().to_string();
    let tmp_file_name = test_id.clone() + "_test.c";
    let mut tmp_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(tmp_file_name.clone())
        .expect("Failed to open compiler IR file");

    tmp_file
        .write_all(code.as_bytes())
        .expect("Failed to write compiled code to IR file");

    let result = match compiler {
        "tcc" => {
            let result = Command::new("tcc")
                .args(["-c", "-Werror", &tmp_file_name])
                .output()
                .expect("TCC execution failed")
                .status
                .success();
            if result {
                remove_file(test_id + "_test.o").expect("Failed to remove test.o file");
            }
            result
        }
        "gcc" => Command::new("gcc")
            .args(["-fsyntax-only", "-Werror", &tmp_file_name])
            .output()
            .expect("GCC execution failed")
            .status
            .success(),
        "clang" => Command::new("clang")
            .args(["-fsyntax-only", "-Werror", &tmp_file_name])
            .output()
            .expect("Clang execution failed")
            .status
            .success(),
        _ => false,
    };

    remove_file(tmp_file_name).expect("Failed to remove TMP file");

    result
}

#[test]
fn hello_world() {
    let code = compile(
        r#"
fn main {
    () helloworld
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn print_line() {
    let code = compile(
        r#"
fn main {
    () println "test"
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn vars() {
    let code = compile(
        r#"
fn main {
    var int number 5
    = number 7
    () println %d number
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn fn_sum() {
    let code = compile(
        r#"
fn test int num1 int num2 > int {
    ret num1 + num2
}

fn main {
    var int sum () test 1 5
    () println %d sum
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn fn_void() {
    let code = compile(
        r#"
fn voidfn {
}
fn main {
    () voidfn
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn fn_args_no_ret() {
    let code = compile(
        r#"
fn argsfn int arg1 {
}
fn main {
    () argsfn 10
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn fn_ret_int() {
    let code = compile(
        r#"
fn intret > int {
    ret 0
}
fn main {
    var int var1 () intret
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn fn_args_ret() {
    let code = compile(
        r#"
fn argsret int arg1 > int {
    ret arg1
}
fn main {
    var int var1 () argsret 10
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn c_var() {
    let code = compile(
        r#"
fn main {
C   int var = 1;
    () println %d var
}
"#,
    );
    assert!(test_code(&code));
}

#[test]
fn c_func() {
    let code = compile(
        r#"
C int func() {
C     return 0;
C }
export func 0

fn main {
    var int result () func
    () println %d result
}
"#,
    );
    assert!(test_code(&code));
}
