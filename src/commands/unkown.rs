// Std Imports
use std::sync::Arc;

// Library Imports
use anyhow::Result;
use async_trait::async_trait;

use crate::utils::App;

// Super Imports
use super::Command;

pub struct Compile {}

#[async_trait]
impl Command for Compile {
    fn help() -> String {
        todo!()
    }

    async fn exec(app: Arc<App>) -> Result<()> {
        let args = app.args.clone();
        let command: &str = args[0].as_str();
        println!("{:#?}", command);
        Ok(())
    }
}
