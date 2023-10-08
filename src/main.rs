mod defs;
mod producer;
mod orchestrator;
mod local_consumer;

use clap::Parser;
use orchestrator::SimpleOrchestrator;

#[derive(Parser, Debug)]
#[command(author, version, about = "Find your zombie!", long_about = None)]
pub struct Args {
    /// The directory to search in.
    #[arg(short = 'd', long = "dir", required = true)]
    dir: String,

    /// The query (file/dir name) to search for.
    #[arg(short = 'q', long = "query", required = true)]
    query: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let root_dir = std::path::PathBuf::from(args.dir);

    if !root_dir.is_dir() {
        println!("This is not a directory!\nEntered Directory: {}", root_dir.to_str().unwrap());
    }

    // Create the orchestrator.
    let orchestrator = SimpleOrchestrator::new(args.query, root_dir);

    // Run the orchestrator.
    orchestrator.run().await;
    Ok(())
}