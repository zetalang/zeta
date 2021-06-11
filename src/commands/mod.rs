use std::str::FromStr;
use std::sync::Arc;

// Library Imports
use anyhow::Result;
use async_trait::async_trait;

// Crate Level Imports
use crate::utils::App;

// Modules
pub mod help;
pub mod unkown;
#[derive(Debug)]
pub enum AppCommand {
    Unknown,
    Help,
}

impl FromStr for AppCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "compile" => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

impl AppCommand {
    pub fn current() -> Option<Self> {
        if std::env::args().len() == 1 {
            return Some(Self::Help);
        }

        match std::env::args().nth(1) {
            Some(cmd) => Self::from_str(cmd.as_str()).ok(),
            None => None,
        }
    }

    pub fn help(&self) -> String {
        match self {
            Self::Unknown => unkown::Compile::help(),
            Self::Help => help::Help::help(),
        }
    }

    pub async fn run(&self, app: App) -> Result<()> {
        let app = Arc::new(app);
        match self {
            Self::Unknown => unkown::Compile::exec(app).await,
            Self::Help => help::Help::exec(app).await,
        }
    }
}

#[async_trait]
pub trait Command {
    fn help() -> String;

    async fn exec(app: Arc<App>) -> Result<()>;
}
