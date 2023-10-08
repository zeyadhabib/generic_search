mod handler;

use clap::Parser;
use tokio::sync::mpsc;
use std::sync::Arc;

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
async fn main() {
    // TODO: Add comments
    let args = Args::parse();

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