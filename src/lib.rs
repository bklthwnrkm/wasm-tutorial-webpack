extern crate wasm_bindgen;

// This is supposed to be like #[wasm_bindgen::prelude::wasm_bindgen] but can be shortened by using "use"
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// "relative module paths aren't supported yet.." error occurred. Changed the relative path to an absolute one.
#[wasm_bindgen(module = "C:/Users/viz-job/Documents/wasm-tutorial-webpack/src/index.js")]
extern {
    fn date_now() -> f64;
    fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn get_timestamp() -> f64 {
    date_now()
}

extern crate tinymt;

use tinymt::tinymt32;

#[wasm_bindgen]
pub fn rand() -> u32 {
    let param = tinymt32::Param {
        mat1: 0x8F7011EE,
        mat2: 0xFC78FF1F,
        tmat: 0x3793fdff,
    };
    let seed = 1;
    tinymt32::from_seed(param, seed).gen()
}

#[wasm_bindgen]
pub fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

#[wasm_bindgen]
pub fn twice(slice: &[i32]) -> Vec<i32> {
    slice.iter().map(|x| x * 2).collect()
}

#[wasm_bindgen]
pub fn hello() {
    console_log("Hello, World!");
}
