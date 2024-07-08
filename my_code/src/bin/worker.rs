use gloo_worker::Registrable;
use wasm_worker_gloo::Square;

fn main() {
    // console_error_panic_hook::set_once();
    Square::registrar().register();
}
