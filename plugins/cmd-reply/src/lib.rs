use kovi::PluginBuilder as plugin;
use qqbot_core::{
    BOT_CACHE, StrategeType, UserData,
    config::{APPCONFIG, get_db},
    reply_strategy::{
        Env, MessageContent, MessageContext, RelyStrategy, cmd::CommandReplyStrategy,
    },
};
#[kovi::plugin]
async fn main() {
    let bot = plugin::get_runtime_bot();
    get_db().await;
    plugin::on_msg(move |event| {
        let bot = bot.clone();
        async move {
            let sender = event.sender.user_id;
            let data = if let Some(user_data) = BOT_CACHE.get(&sender).await {
                user_data
            } else {
                BOT_CACHE.insert(sender, UserData::default()).await;
                UserData::default()
            };
            match data.stratege {
                StrategeType::CmdStrategy => {
                    if let Some(msg) = event.borrow_text() {
                        if msg.starts_with(&APPCONFIG.cmd_suffix) {
                            let env = if event.message_type == Env::Private.to_string() {
                                Env::Private
                            } else {
                                Env::Group
                            };
                            let mc = MessageContent::Text(msg.to_string());
                            let message_context = MessageContext {
                                env: env,
                                sender_id: event.sender.user_id,
                                self_id: event.self_id,
                                message: mc,
                                history: vec![],
                            };
                            let cmd_strategy = CommandReplyStrategy::new();
                            let reply_msg = match cmd_strategy.reply(&message_context).await {
                                Ok(MessageContent::Text(res)) => res,
                                Err(err) => err.to_string(),
                                _ => String::from("unsuppoted reply message"),
                            };
                            bot.send_private_msg(event.sender.user_id, reply_msg);
                        }
                    }
                }
            }
        }
    });
}
