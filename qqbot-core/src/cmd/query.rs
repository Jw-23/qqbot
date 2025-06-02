use std::pin::Pin;

// 1. 修正 use 语句：不需要 clap_derive::Parser，只需要 clap::Parser trait
use crate::{
    config::DB_GLOBAL,
    service::grade_service::{GradeService, GradeServiceImpl},
};
use clap::{Parser, Subcommand, ValueEnum};

use super::{CmdHandler, CmdResult, CommonArgs, HandlerBuilder}; // 移除 clap_derive::Parser
use crate::error::AppError;

#[derive(Parser, Debug)]
#[command(
    name = "query", // 程序名，会用于匹配 args 的第一个元素
    author = "jw23",
    version = "0.1",
    about = "query grade for students"
)]
pub struct Query {
    #[command(flatten)]
    pub common: CommonArgs,
    #[command(subcommand)]
    commands: QueryItem, // 子命令字段
}
// impl Query {
//     pub fn new() -> Self {
//         Query {
//             commands: QueryItem::Grade {
//                 id: 0,
//                 mode: GradeQueryMode::Summary,

//             },
//             sender_id: 0,
//             self_id: 0,
//         }
//     }
// }
#[derive(Subcommand, Debug)]
#[command(about = "query grade")]
pub enum QueryItem {
    /// the subcommand to query grade
    Grade {
        /// 查询模式 (必需, 默认 Summary, 忽略大小写)
        #[arg(
            short,
            long,
            // 实际上因为有 default_value_t，required=true 不是必需的，但写上无妨
            ignore_case = true,
            value_enum,
            default_value_t = GradeQueryMode::Summary, // 提供默认值
            help="mode: Summary,Quiv1,Quiv2,Quiv3,Quiv4,Mid"
        )]
        mode: GradeQueryMode,
    },
    // 如果有其他子命令，可以继续在这里添加
    // OtherSubcommand { ... }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum GradeQueryMode {
    Summary,
    Quiv1, // 注意：clap 会将驼峰命名转为小写连字符形式 (如 quiv1) 或直接小写
    Quiv2,
    Quiv3,
    Quiv4,
    Mid,
}
impl HandlerBuilder for Query {
    fn build() -> CmdHandler {
        Box::new(move |args: Vec<String>| {
            // 将 async 块的结果明确赋值给一个变量
            let fut = async move {
                let query = Query::try_parse_from(args).map_err(|err| AppError::command(err.to_string()))?;
                match query.commands {
                    QueryItem::Grade {  mode } => {                        let conn = DB_GLOBAL
                            .get()
                            .ok_or_else(|| AppError::command(String::from("failed to connect database")))?;

                        let grade_repo = GradeServiceImpl::new(conn.clone()); // 确保 conn 可以 clone
                        let grades = grade_repo
                        .find_grades(query.common.sender)
                        .await
                        .map_err(|err|AppError::command(format!("bind your student number with /bind (id) first:{}",err.to_string())))?;

                        match mode {
                            GradeQueryMode::Summary => {
                                // 优化: 使用 String::new() 和 push_str 效率更高
                                let mut report_str = String::new();
                                for record in grades {
                                    // 假设 record 有 student_name, exam_name, score 字段
                                    use std::fmt::Write; // 引入 Write trait 以使用 write! 宏
                                    // 使用 write! 宏避免多次分配 String

                                    write!(
                                         &mut report_str,
                                         "{} 在 {} 中获得 {} 分;\n",
                                         record.student_name, record.exam_name, record.score
                                     ).map_err(|_| AppError::command(String::from("failed to format report")))?; // 处理 write! 可能的错误
                                     if record.score<60 {
                                         write!(&mut report_str,"请多花时间，微积分对未来的课程非常重要，务必掌握\n").map_err(|_| AppError::command(String::from("failed to format report")))?; // 处理 write! 可能的错误
                                     }
                                }
                                // 移除不必要的字节转换和 UTF-8 转换
                                Ok(CmdResult { output: report_str })
                            }
                            _ => Err(AppError::command(format!("Query mode {:?} not supported yet.", mode))), // 提供更具体的错误信息
                        }
                    }
                     // 如果 QueryItem 有其他变体，需要在这里处理，否则 _ 可能匹配到非预期的命令
                     // 如果 Grade 是唯一的子命令，这个 _ 分支实际上是不可达的，除非解析逻辑有误
                     // 或者可以移除 new() 方法和默认值，让 clap 强制必须提供子命令
                     // _ => Err(CmdError("Invalid subcommand specified.".into())),
                }
            }; // fut 的类型是具体的匿名 Future

            // **关键改动**:
            // 1. Box::pin 这个 future
            // 2. 显式地将 Pin<Box<ConcreteFuture>> 转换为 Pin<Box<dyn Future + Send>>
            //    Rust 通常允许这种从具体类型到 trait object 的隐式强制转换（coercion），
            //    尤其是在赋值或返回时，如果目标类型是 trait object。
            //    我们可以显式声明类型来帮助编译器。

            // 方案 A：显式类型注解 (推荐)
            let handler_fut: Pin<Box<dyn Future<Output = Result<CmdResult, AppError>> + Send>> = Box::pin(fut);
            handler_fut // 返回这个明确类型的 pinned future

            // 方案 B：直接返回并依赖类型推断（有时可行，但出错时退回方案 A）
            // Box::pin(fut)
            // 如果方案 B 仍然报错，说明编译器需要更明确的指示，应使用方案 A

        }) // `as CmdHandler` 现在应该能工作，因为闭包返回的类型与 CmdHandler 定义匹配
           // 注意: 这里的 as CmdHandler 是将 Box<dyn Fn(...)> 转换为 CmdHandler (类型别名)
           // 如果 CmdHandler 就是 Box<dyn Fn(...)> 类型，这个 as 甚至可能不需要。
           // 但保留它通常无害，可以明确意图。
           as CmdHandler
    }
}
