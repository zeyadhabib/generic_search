use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
use colored::*;

type DataSender = mpsc::Sender<PathBuf>;
type TxSender = mpsc::Sender<DataSender>;

pub async fn process (path: PathBuf, query: Arc<String>, tx: mpsc::Sender<PathBuf>, sender_tx: TxSender) {
    let mut handles = Vec::new();

    if path.to_str().unwrap().contains(query.as_str()) {
        if path.is_file() {
            println!("{} {}", "[FIL] ".green(), path.to_str().unwrap().to_string().green());
        } else if path.is_dir() {
            println!("{} {}", "[DIR] ".green().blink(), path.to_str().unwrap().to_string().green().blink());
        }
    }

    if !path.is_dir() {
        return;
    }
    match path.read_dir() {
        Ok(contents)=>{
            for file in contents {
                match file {
                    Ok(file)=>{
                        let tx_clone = tx.clone();
                        let tx_sender_clone = sender_tx.clone();
                        let handle = tokio::spawn(
                            async move {
                                let _ = tx_clone.send(file.path()).await;
                                let _ = tx_sender_clone.send(tx_clone).await;
                            }
                        );
                        handles.push(handle);
                    },
                    Err(_)=>{}
                }
            }
        },
        Err(_)=>{}
    }
    futures::future::join_all(handles).await;
}