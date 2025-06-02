use std::time::Duration;

use kovi::{
    PluginBuilder as plugin,
    log::warn,
    tokio::time::timeout,
};

#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    plugin::on_all_request(move |event| {
        let bot = bot.clone();
        let event = event.clone();
        async move {
            if event.request_type == "friend" {
                let flag = event.original_json.get("flag").unwrap().as_str().unwrap();
                let user_id = event.original_json.get("user_id").unwrap().as_i64().unwrap();
            
                bot.set_friend_add_request(flag, true, "");
                let res = timeout(Duration::from_secs(3), async {
                    bot.send_private_msg(user_id, "welcome to use wbot");
                })
                .await;
                if let Err(err) = res {
                    warn!(
                        "failed to send message after approval friend request: {}",
                        err
                    );
                }
            }
        }
    });
}
