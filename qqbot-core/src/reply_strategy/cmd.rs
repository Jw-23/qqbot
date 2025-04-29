use super::{MessageContent, MessageContext, RelyStrategy, ReplyError};
use crate::cmd::{CMD_REGISTRY, Execute};
use crate::config::APPCONFIG;

pub struct CommandReplyStrategy {}

impl CommandReplyStrategy {
    pub fn new() -> Self {
        Self {}
    }
}

impl RelyStrategy for CommandReplyStrategy {
    async fn reply(&self, ctx: &MessageContext) -> Result<MessageContent, ReplyError> {
        
        match &ctx.message {
            
            MessageContent::Text(cmd) => {
                let args: Vec<&str> = cmd.split_whitespace().filter(|arg| !arg.trim().is_empty()).collect();
                let cmd = if args.len() > 0 {
                    args[0].strip_prefix(&APPCONFIG.cmd_suffix).ok_or_else(|| {
                        ReplyError(format!(
                            "expected command suffix \"{}\"",
                            APPCONFIG.cmd_suffix
                        ))
                    })
                } else {
                    return Err(ReplyError(format!(
                        "the message isn't command, expected suffix {}",
                        APPCONFIG.cmd_suffix
                    )));
                }?;
                
                let mut args = args[1..].to_vec();
                args.push("--sender");
                let send_id = ctx.sender_id.to_string();
                args.push(&send_id);
                args.push("--myself");
                let myself = ctx.self_id.to_string();
                args.push(&myself);
                args.push("--env");
                let env = ctx.env.to_string();
                args.push(&env);
                let cmd_result = CMD_REGISTRY
                    .execute(cmd, &args)
                    .await
                    .map_err(|err| ReplyError(err.to_string()))?;
                return Ok(MessageContent::Text(cmd_result.output));
            }
            _ => Err(ReplyError("only support text command message".into())),
        }
    }
}
