use gloo_worker::oneshot::oneshot;
use gloo_worker::Spawnable;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;

#[oneshot]
pub async fn Square(input: i32) -> i32 {
    let result = input.pow(2);
    return result;
}

#[wasm_bindgen]
pub async fn square_worker(input: i32) -> i32{
    let mut spawn_worker = Square::spawner().spawn("./workspace.js");
    let response = spawn_worker.run(input).await;
    return response
}
