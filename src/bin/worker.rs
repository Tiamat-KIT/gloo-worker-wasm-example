use gloo_worker::Registrable;
use workspace::Square;

fn main() {
    Square::registrar().register();
}
