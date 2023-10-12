mod zombie_hunt_client;

use colored::*;
use clap::Parser;
use zombie_hunt_client::run;
use generic_search::local_orchestrator::SimpleOrchestrator;

#[derive(Parser, Debug)]
#[command(author, version, about = "Find your zombie!", long_about = None)]
pub struct Args {
    /// The directory to search in.
    #[arg(short = 'd', long = "dir", required = true)]
    dir: String,

    /// The query (file/dir name) to search for.
    #[arg(short = 'q', long = "query", required = true)]
    query: String,

    /// Switch to enable remote search.
    #[arg(short = 'r', long = "remote", required = false, default_value = "false")]
    remote: bool,

    /// Switch to enable remote search.
    #[arg(long = "remote-only", required = false, default_value = "false")]
    remote_only: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let root_dir = std::path::PathBuf::from(args.dir.clone());

    if !args.remote && args.remote_only {
        println!("{}", "You just wasted precious CPU cycles!".red().bold());
        println!("{}", "Please select at least one search method (remote/local)... Simpleton.".red().bold());
        return Ok(());
    }

    if !root_dir.is_dir() {
        println!("{} {}", "This is not a directory!\nEntered Directory:".red().bold(), root_dir.to_str().unwrap().red().bold());
    }

    if !args.remote_only {
        // Create the orchestrator.
        let orchestrator = SimpleOrchestrator::new(args.query.clone(), root_dir);
        // Run the orchestrator.
        orchestrator.run().await;
    }

    if args.remote {
        // Run the client.
        run("zeyad.server.com", "https://[::1]", 50051, args.dir, args.query).await?;
    }

    Ok(())
}