use kovi::build_bot;
use reply;
fn main() {
    build_bot!(reply, apply_request).run();
}
