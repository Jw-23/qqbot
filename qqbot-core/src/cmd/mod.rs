pub mod bind;
pub mod query;
pub mod strategy;
use bind::Bind;
use once_cell::sync::Lazy;
use query::Query;
use strategy::Strategy;
// Assuming query module exists and defines Query structuse clap::Parser;
use crate::error::AppError;
use clap::Parser;
use std::{collections::HashMap, future::Future, pin::Pin}; // Added Arc]

// --- Data Structures and Errors (Keep as is) ---
pub type CmdRegistry = HashMap<String, CmdHandler>; // Use Arc for potential sharing
#[derive(Parser, Debug, Clone)]
pub struct CommonArgs {
    #[arg(long, global = true, required = false, help = "sender id(auto)")]
    sender: i64,
    #[arg(long, global = true, required = false, help = "self id(auto)")]
    myself: i64,
    #[arg(long,global = true,required = false, help = "env private, group(auto)",default_value_t=String::from("private"))]
    env: String,
    #[arg(
        long,
        global = true,
        required = false,
        help = "是否是群管理(auto)",
        default_value_t = false
    )]
    group_admin: bool,
}
#[derive(Debug)] // Added Debug for easier printing
pub struct CmdResult {
    pub output: String,
    // 可以扩展更多字段，如状态码等
}

type CmdHandler = Box<
    dyn Fn(Vec<String>) -> Pin<Box<dyn Future<Output = Result<CmdResult, AppError>> + Send>>
        + Sync
        + Send,
>;

trait HandlerBuilder {
    fn build() -> CmdHandler;
}

// --- Execute Trait (Keep as is) ---
pub trait Execute {
    // Takes &Vec<&str> which is fine for passing arguments *to* execute
    fn execute(
        &self,
        cmd: &str,
        args: &Vec<&str>,
    ) -> impl std::future::Future<Output = Result<CmdResult, AppError>> + Send;
}

// --- Implementation of Execute for CmdRegistry ---
impl Execute for CmdRegistry {
    async fn execute(&self, cmd: &str, args: &Vec<&str>) -> Result<CmdResult, AppError> {
        // 1. Find the handler in the registry
        if let Some(handler) = self.get(cmd) {
            // 2. Prepare arguments for the handler's `run` method.
            //    The `run` method now expects `Vec<String>`.
            //    We convert the input `&Vec<&str>` to `Vec<String>`.
            //    We also need to prepend the command name itself, as clap often expects it.
            let mut full_args: Vec<String> = Vec::with_capacity(args.len() + 1);
            full_args.push(cmd.into()); // Add command name as first arg for clap parsing
            full_args.extend(args.iter().map(|s| s.to_string())); // Convert &str to String

            // 3. Call the handler's `run` instance method
            //    Since handler is Arc<dyn CmdHandler<T>>, we call run on the dereferenced trait object.
            let future = handler(full_args);

            // 4. Await the future returned by run
            future.await // This returns Result<CmdResult, AppError>
        } else {
            // 5. Handle case where the command is not found
            Err(AppError::command(format!("Command '{}' not found", cmd)))
        }
    }
}
pub static CMD_REGISTRY: Lazy<CmdRegistry> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("query".into(), Query::build());
    m.insert("bind".into(), Bind::build());
    m.insert("strategy".into(), Strategy::build());
    m
});
