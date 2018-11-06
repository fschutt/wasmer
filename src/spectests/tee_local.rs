// Rust test file autogenerated with cargo build (src/build_spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/tee_local.wast
#![allow(
    warnings,
    dead_code
)]
use std::panic;
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, Instance, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 3
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (type (;1;) (func (result i64)))
      (type (;2;) (func (result f32)))
      (type (;3;) (func (result f64)))
      (type (;4;) (func (param i32) (result i32)))
      (type (;5;) (func (param i64) (result i64)))
      (type (;6;) (func (param f32) (result f32)))
      (type (;7;) (func (param f64) (result f64)))
      (type (;8;) (func (param i64 f32 f64 i32 i32)))
      (type (;9;) (func (param i64 f32 f64 i32 i32) (result i64)))
      (type (;10;) (func (param i64 f32 f64 i32 i32) (result f64)))
      (func (;0;) (type 0) (result i32)
        (local i32)
        i32.const 0
        tee_local 0)
      (func (;1;) (type 1) (result i64)
        (local i64)
        i64.const 0
        tee_local 0)
      (func (;2;) (type 2) (result f32)
        (local f32)
        f32.const 0x0p+0 (;=0;)
        tee_local 0)
      (func (;3;) (type 3) (result f64)
        (local f64)
        f64.const 0x0p+0 (;=0;)
        tee_local 0)
      (func (;4;) (type 4) (param i32) (result i32)
        i32.const 10
        tee_local 0)
      (func (;5;) (type 5) (param i64) (result i64)
        i64.const 11
        tee_local 0)
      (func (;6;) (type 6) (param f32) (result f32)
        f32.const 0x1.633334p+3 (;=11.1;)
        tee_local 0)
      (func (;7;) (type 7) (param f64) (result f64)
        f64.const 0x1.8666666666666p+3 (;=12.2;)
        tee_local 0)
      (func (;8;) (type 8) (param i64 f32 f64 i32 i32)
        (local f32 i64 i64 f64)
        i64.const 0
        tee_local 0
        i64.eqz
        drop
        f32.const 0x0p+0 (;=0;)
        tee_local 1
        f32.neg
        drop
        f64.const 0x0p+0 (;=0;)
        tee_local 2
        f64.neg
        drop
        i32.const 0
        tee_local 3
        i32.eqz
        drop
        i32.const 0
        tee_local 4
        i32.eqz
        drop
        f32.const 0x0p+0 (;=0;)
        tee_local 5
        f32.neg
        drop
        i64.const 0
        tee_local 6
        i64.eqz
        drop
        i64.const 0
        tee_local 7
        i64.eqz
        drop
        f64.const 0x0p+0 (;=0;)
        tee_local 8
        f64.neg
        drop)
      (func (;9;) (type 9) (param i64 f32 f64 i32 i32) (result i64)
        (local f32 i64 i64 f64)
        f32.const -0x1.333334p-2 (;=-0.3;)
        tee_local 1
        drop
        i32.const 40
        tee_local 3
        drop
        i32.const -7
        tee_local 4
        drop
        f32.const 0x1.6p+2 (;=5.5;)
        tee_local 5
        drop
        i64.const 6
        tee_local 6
        drop
        f64.const 0x1p+3 (;=8;)
        tee_local 8
        drop
        get_local 0
        f64.convert_u/i64
        get_local 1
        f64.promote/f32
        get_local 2
        get_local 3
        f64.convert_u/i32
        get_local 4
        f64.convert_s/i32
        get_local 5
        f64.promote/f32
        get_local 6
        f64.convert_u/i64
        get_local 7
        f64.convert_u/i64
        get_local 8
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        i64.trunc_s/f64)
      (func (;10;) (type 10) (param i64 f32 f64 i32 i32) (result f64)
        (local f32 i64 i64 f64)
        i64.const 1
        tee_local 0
        f64.convert_u/i64
        f32.const 0x1p+1 (;=2;)
        tee_local 1
        f64.promote/f32
        f64.const 0x1.a666666666666p+1 (;=3.3;)
        tee_local 2
        i32.const 4
        tee_local 3
        f64.convert_u/i32
        i32.const 5
        tee_local 4
        f64.convert_s/i32
        f32.const 0x1.6p+2 (;=5.5;)
        tee_local 5
        f64.promote/f32
        i64.const 6
        tee_local 6
        f64.convert_u/i64
        i64.const 0
        tee_local 7
        f64.convert_u/i64
        f64.const 0x1p+3 (;=8;)
        tee_local 8
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add
        f64.add)
      (export \"type-local-i32\" (func 0))
      (export \"type-local-i64\" (func 1))
      (export \"type-local-f32\" (func 2))
      (export \"type-local-f64\" (func 3))
      (export \"type-param-i32\" (func 4))
      (export \"type-param-i64\" (func 5))
      (export \"type-param-f32\" (func 6))
      (export \"type-param-f64\" (func 7))
      (export \"type-mixed\" (func 8))
      (export \"write\" (func 9))
      (export \"result\" (func 10)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject()).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 98
fn c1_l98_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c1_l98_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-local-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 0 as i32);
}

// Line 99
fn c2_l99_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c2_l99_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-local-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 0 as i64);
}

// Line 100
fn c3_l100_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c3_l100_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-local-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 0.0 as f32);
}

// Line 101
fn c4_l101_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c4_l101_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-local-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 0.0 as f64);
}

// Line 103
fn c5_l103_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c5_l103_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-param-i32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i32, &result_object.instance);
    assert_eq!(result, 10 as i32);
}

// Line 104
fn c6_l104_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c6_l104_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-param-i64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(3 as i64, &result_object.instance);
    assert_eq!(result, 11 as i64);
}

// Line 105
fn c7_l105_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c7_l105_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-param-f32") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f32, &Instance) -> f32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(4.4 as f32, &result_object.instance);
    assert_eq!(result, 11.1 as f32);
}

// Line 106
fn c8_l106_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c8_l106_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-param-f64") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(f64, &Instance) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(5.5 as f64, &result_object.instance);
    assert_eq!(result, 12.2 as f64);
}

// Line 109
fn c9_l109_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c9_l109_action_invoke");
    let func_index = match result_object.module.info.exports.get("type-mixed") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, f32, f64, i32, i32, &Instance) = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i64, 2.2 as f32, 3.3 as f64, 4 as i32, 5 as i32, &result_object.instance);
    assert_eq!(result, ());
}

// Line 115
fn c10_l115_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c10_l115_action_invoke");
    let func_index = match result_object.module.info.exports.get("write") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, f32, f64, i32, i32, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i64, 2.0 as f32, 3.3 as f64, 4 as i32, 5 as i32, &result_object.instance);
    assert_eq!(result, 56 as i64);
}

// Line 122
fn c11_l122_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c11_l122_action_invoke");
    let func_index = match result_object.module.info.exports.get("result") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, f32, f64, i32, i32, &Instance) -> f64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(-1 as i64, -2.0 as f32, -3.3 as f64, -4 as i32, -5 as i32, &result_object.instance);
    assert_eq!(result, 34.8 as f64);
}

// Line 132
#[test]
fn c12_l132_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 0, 1, 126, 3, 2, 1, 0, 10, 10, 1, 8, 1, 1, 127, 65, 0, 34, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 136
#[test]
fn c13_l136_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 14, 1, 12, 1, 1, 125, 67, 0, 0, 0, 0, 34, 0, 69, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 140
#[test]
fn c14_l140_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 2, 1, 124, 1, 126, 66, 0, 34, 1, 154, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 145
#[test]
fn c15_l145_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 9, 1, 7, 1, 1, 127, 1, 34, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 149
#[test]
fn c16_l149_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 1, 1, 127, 67, 0, 0, 0, 0, 34, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 153
#[test]
fn c17_l153_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 17, 1, 15, 1, 1, 125, 68, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 157
#[test]
fn c18_l157_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 19, 1, 17, 2, 1, 124, 1, 126, 68, 0, 0, 0, 0, 0, 0, 0, 0, 34, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 165
#[test]
fn c19_l165_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 1, 127, 1, 126, 3, 2, 1, 0, 10, 6, 1, 4, 0, 32, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 169
#[test]
fn c20_l169_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 125, 0, 3, 2, 1, 0, 10, 7, 1, 5, 0, 32, 0, 69, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 173
#[test]
fn c21_l173_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 124, 126, 0, 3, 2, 1, 0, 10, 7, 1, 5, 0, 32, 1, 154, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 178
#[test]
fn c22_l178_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 10, 7, 1, 5, 0, 1, 34, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 182
#[test]
fn c23_l182_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 10, 11, 1, 9, 0, 67, 0, 0, 0, 0, 34, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 186
#[test]
fn c24_l186_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 125, 0, 3, 2, 1, 0, 10, 15, 1, 13, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 34, 0, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 190
#[test]
fn c25_l190_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 124, 126, 0, 3, 2, 1, 0, 10, 15, 1, 13, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 34, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 198
#[test]
fn c26_l198_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 10, 1, 8, 2, 1, 127, 1, 126, 32, 3, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 202
#[test]
fn c27_l202_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 13, 1, 11, 2, 1, 127, 1, 126, 32, 247, 164, 234, 6, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 207
#[test]
fn c28_l207_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 127, 126, 0, 3, 2, 1, 0, 10, 6, 1, 4, 0, 32, 2, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 211
#[test]
fn c29_l211_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 14, 1, 12, 2, 1, 127, 1, 126, 32, 247, 242, 206, 212, 2, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 216
#[test]
fn c30_l216_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 127, 0, 3, 2, 1, 0, 10, 10, 1, 8, 2, 1, 127, 1, 126, 32, 3, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 220
#[test]
fn c31_l220_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 126, 0, 3, 2, 1, 0, 10, 13, 1, 11, 2, 1, 127, 1, 126, 32, 247, 168, 153, 102, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 225
#[test]
fn c32_l225_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 125, 0, 3, 2, 1, 0, 10, 13, 1, 11, 1, 1, 127, 67, 0, 0, 0, 0, 34, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 229
#[test]
fn c33_l229_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 6, 1, 96, 2, 126, 127, 0, 3, 2, 1, 0, 10, 13, 1, 11, 1, 1, 125, 67, 0, 0, 0, 0, 34, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

// Line 233
#[test]
fn c34_l233_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 5, 1, 96, 1, 126, 0, 3, 2, 1, 0, 10, 12, 1, 10, 2, 1, 124, 1, 126, 66, 0, 34, 1, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    println!("Running tests in file: {:?}", file!());
    let result_object = create_module_1();
    // We group the calls together
    start_module_1(&result_object);
    c1_l98_action_invoke(&result_object);
    c2_l99_action_invoke(&result_object);
    c3_l100_action_invoke(&result_object);
    c4_l101_action_invoke(&result_object);
    c5_l103_action_invoke(&result_object);
    c6_l104_action_invoke(&result_object);
    c7_l105_action_invoke(&result_object);
    c8_l106_action_invoke(&result_object);
    c9_l109_action_invoke(&result_object);
    c10_l115_action_invoke(&result_object);
    c11_l122_action_invoke(&result_object);
}
