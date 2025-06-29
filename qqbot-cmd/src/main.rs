use kovi::build_bot;

fn main() {
    build_bot!(reply, apply_request, push).run();
}
