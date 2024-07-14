use clap::Parser;
use reqwest::Result;

mod modules;

use modules::app::App;
use modules::app::RunArgs;
use modules::config::AppConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let config = AppConfig::load();
    let arguments = RunArgs::parse();
    let app = App { config: &config };

    app.run(&arguments).await
}
