use kovi::build_bot;
use cmd_reply;
fn main() {
    build_bot!(cmd_reply,apply_request,kovi_plugin_siliconflow).run();
}
