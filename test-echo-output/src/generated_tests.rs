//! This file is generated by build.rs
//! Do not edit it directly, instead add new test cases to ./cases

#[rustfmt::skip]
#[test]
fn echo_bitarray() {
    let output = crate::prepare("./cases/echo_bitarray");
    insta::assert_snapshot!(
        "echo_bitarray",
        output,
        "./cases/echo_bitarray",
    );
}

#[rustfmt::skip]
#[test]
fn echo_bool() {
    let output = crate::prepare("./cases/echo_bool");
    insta::assert_snapshot!(
        "echo_bool",
        output,
        "./cases/echo_bool",
    );
}

#[rustfmt::skip]
#[test]
fn echo_custom_type() {
    let output = crate::prepare("./cases/echo_custom_type");
    insta::assert_snapshot!(
        "echo_custom_type",
        output,
        "./cases/echo_custom_type",
    );
}

#[rustfmt::skip]
#[test]
fn echo_dict() {
    let output = crate::prepare("./cases/echo_dict");
    insta::assert_snapshot!(
        "echo_dict",
        output,
        "./cases/echo_dict",
    );
}

#[rustfmt::skip]
#[test]
fn echo_float() {
    let output = crate::prepare("./cases/echo_float");
    insta::assert_snapshot!(
        "echo_float",
        output,
        "./cases/echo_float",
    );
}

#[rustfmt::skip]
#[test]
fn echo_function() {
    let output = crate::prepare("./cases/echo_function");
    insta::assert_snapshot!(
        "echo_function",
        output,
        "./cases/echo_function",
    );
}

#[rustfmt::skip]
#[test]
fn echo_int() {
    let output = crate::prepare("./cases/echo_int");
    insta::assert_snapshot!(
        "echo_int",
        output,
        "./cases/echo_int",
    );
}

#[rustfmt::skip]
#[test]
fn echo_list() {
    let output = crate::prepare("./cases/echo_list");
    insta::assert_snapshot!(
        "echo_list",
        output,
        "./cases/echo_list",
    );
}

#[rustfmt::skip]
#[test]
fn echo_nil() {
    let output = crate::prepare("./cases/echo_nil");
    insta::assert_snapshot!(
        "echo_nil",
        output,
        "./cases/echo_nil",
    );
}

#[rustfmt::skip]
#[test]
fn echo_string() {
    let output = crate::prepare("./cases/echo_string");
    insta::assert_snapshot!(
        "echo_string",
        output,
        "./cases/echo_string",
    );
}

#[rustfmt::skip]
#[test]
fn echo_tuple() {
    let output = crate::prepare("./cases/echo_tuple");
    insta::assert_snapshot!(
        "echo_tuple",
        output,
        "./cases/echo_tuple",
    );
}
