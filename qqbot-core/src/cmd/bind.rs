use std::fmt::format;

use clap::Parser;

use crate::{
    cmd::{CMD_REGISTRY, Execute},
    config::{DB_GLOBAL, get_db},
    permission::check_permission,
    reply_strategy::Env,
    repo::student::StudentRepo,
    service::{StuServiceImpl, UserService},
};

use super::{CmdError, CmdHandler, CmdResult, CommonArgs, HandlerBuilder};
#[derive(Debug, Clone, Parser)]
pub struct Bind {
    #[command(flatten)]
    pub common:CommonArgs,
    #[arg(long, help = "clear qq", default_value_t = false)]
    clear: bool,
    #[arg(required = false, help = "student number", default_value_t = 0)]
    id: i64,
}

impl HandlerBuilder for Bind {
    fn build() -> CmdHandler {
        Box::new(move |args: Vec<String>| {
            Box::pin(async move {
                let bind = Bind::try_parse_from(args).map_err(|err| CmdError(err.to_string()))?;
                if bind.common.env != Env::Private.to_string() {
                    return Err(CmdError("only used in private environment".into()));
                }
                let db = DB_GLOBAL
                    .get()
                    .ok_or_else(|| CmdError("failed to connect database".into()))?;
                let ss = StuServiceImpl::new(db.clone());
                if let Ok(model) = ss.find_by_qq(bind.common.sender).await {
                    if bind.clear {
                        ss.update_qq(model.student_id, 0)
                            .await
                            .map_err(|_| CmdError("success".into()))?;
                        return Ok(CmdResult {
                            output: String::from("clear qq number successfully"),
                        });
                    }
                    return Err(CmdError(format!(
                        "the student has been bind to {}",
                        model.student_id
                    )));
                };
                if let Ok(model) = ss.get(bind.id).await {
                    if model.qq_number != 0 {
                        return Err(CmdError(format!(
                            "can't bind the student, because he has been bind to other qq"
                        )));
                    }
                }
                ss.update_qq(bind.id, bind.common.sender)
                    .await
                    .map_err(|err| CmdError(err.to_string()))?;
                Ok(CmdResult {
                    output: "success".into(),
                })
            })
        })
    }
}

#[tokio::test]
async fn bind_test() -> Result<(), Box<dyn std::error::Error>> {
    get_db().await;
    let cmd: Vec<&str> = vec!["bind", "666888"];
    let reports = CMD_REGISTRY.execute(cmd[0], &cmd[1..].to_vec()).await;
    let output = match reports {
        Ok(result) => result.output,
        Err(err) => err.to_string(),
    };
    println!("命令行结果:\n{}", output);
    Ok(())
}
