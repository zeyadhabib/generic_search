mod handler;

use colored::*;
use clap::Parser;
use tokio::sync::mpsc;
use std::{process::exit, sync::Arc};

#[derive(Parser, Debug)]
#[command(author, version, about = "Find your zombie!", long_about = None)]
pub struct Args {
    /// The directory to search in.
    #[arg(index = 1, short, long)]
    dir: String,

    /// The query (file/dir name) to search for.
    #[arg(index = 2, short, long)]
    query: String,

    /// Ignore case when searching.
    #[arg(long, default_value = "false")]
    ignore_case: bool,

    /// Only print files, do not use with only_dirs!.
    #[arg(long, default_value = "false")]
    only_files: bool,

    /// Only print directories, do not use with only_files!.
    #[arg(long, default_value = "false")]
    only_dirs: bool,

    /// Ignore read file/dir errors.
    #[arg(long, short, default_value = "false")]
    ignore_errors: bool,
}

#[tokio::main]
async fn main() {
    // TODO: Add comments
    let args = Args::parse();

    if args.only_dirs && args.only_files {
        println!("{}", "You can't use both --only-files and --only-dirs".red());
        exit(1);
    }

    let query = Arc::new(args.query);
    let root_path = std::path::PathBuf::from(args.dir);
    if !root_path.is_dir() {
        println!("{} is not a directory!", root_path.to_str().unwrap());
        return;
    }

    let (data_tx, mut data_rx) = mpsc::channel(32);
    let (sender_tx, mut sender_rx) = mpsc::channel(32);

    let query_clone = query.clone();
    let sender_tx_clone = sender_tx.clone();
    tokio::spawn(async move {
        handler::process(root_path, query_clone, data_tx, sender_tx_clone).await;
    });

    loop {
        let path_res = data_rx.recv().await;
        match path_res {
            Some(path)=>{
                let query_clone = query.clone();
                let data_tx = sender_rx.recv().await.unwrap();
                let sender_tx_clone = sender_tx.clone();

                tokio::spawn(async move {
                    handler::process(path, query_clone, data_tx, sender_tx_clone).await;
                });
            },
            None=>{
                break;
            }
        }
    }
}