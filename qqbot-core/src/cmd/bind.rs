
use clap::Parser;

use crate::{
    config::DB_GLOBAL,
    reply_strategy::Env,
    service::{StuServiceImpl, UserService},
};

use super::{CmdHandler, CmdResult, CommonArgs, HandlerBuilder};
use crate::error::AppError;
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
                let bind = Bind::try_parse_from(args).map_err(|err| AppError::command(err.to_string()))?;
                if bind.common.env != Env::Private.to_string() {
                    return Err(AppError::command(String::from("only used in private environment")));
                }
                let db = DB_GLOBAL
                    .get()
                    .ok_or_else(|| AppError::command(String::from("failed to connect database")))?;
                let ss = StuServiceImpl::new(db.clone());
                if let Ok(model) = ss.find_by_qq(bind.common.sender).await {
                    if bind.clear {
                        ss.update_qq(model.student_id, 0)
                            .await
                            .map_err(|_| AppError::command(String::from("success")))?;
                        return Ok(CmdResult {
                            output: String::from("clear qq number successfully"),
                        });
                    }
                    return Err(AppError::command(format!(
                        "the student has been bind to {}",
                        model.student_id
                    )));
                };
                if let Ok(model) = ss.get(bind.id).await {
                    if model.qq_number != 0 {
                        return Err(AppError::command(format!(
                            "can't bind the student, because he has been bind to other qq"
                        )));
                    }
                }
                ss.update_qq(bind.id, bind.common.sender)
                    .await
                    .map_err(|err| AppError::command(err.to_string()))?;
                Ok(CmdResult {
                    output: "success".into(),
                })
            })
        })
    }
}

