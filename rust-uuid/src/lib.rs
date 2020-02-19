extern crate uuid;
mod utils;
use uuid::Uuid;

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn gen_v4() -> String {
  let v4 = Uuid::new_v4();
  v4.to_hyphenated().to_string()
}
