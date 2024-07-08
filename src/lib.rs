use gloo_worker::oneshot::oneshot;
use gloo_worker::Spawnable;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

#[oneshot]
pub async fn Square(input: u32) -> u32 {
    let result = input.pow(2);
    return result;
}

#[wasm_bindgen]
pub async fn square_worker(input: u32) {
    let mut spawn_worker = Square::spawner().spawn("path");
    spawn_worker.run(input).await;
}
