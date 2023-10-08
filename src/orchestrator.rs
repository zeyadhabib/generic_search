use std::path::PathBuf;
use tokio::sync::mpsc;

use crate::{producer::SimpleProducer, defs::DirContent, local_consumer::LocalConsumer};

pub struct SimpleOrchestrator {
    query: String,
    root_dir: PathBuf,
}

impl SimpleOrchestrator {

    pub fn new(query: String, root_dir: PathBuf) -> Self {
        Self {
            query,
            root_dir,
        }
    }

    pub async fn run(&self) {

        let (pulse_transmitter, mut pulse_reciever) = mpsc::channel(32);
        let (dir_content_transmitter, mut dir_content_reciever) = mpsc::channel(32);

        let dir_content_transmitter_clone = dir_content_transmitter.clone();
        let root_dir = DirContent::Dir(self.root_dir.clone());

        tokio::spawn(async move {
            SimpleProducer::produce(root_dir, pulse_transmitter, dir_content_transmitter_clone).await;
        });

        loop {
            let pulse_option = pulse_reciever.recv().await;
            match pulse_option {
                Some(_)=>{
                    let (dir_content, pulse_transmitter) = dir_content_reciever.recv().await.unwrap();
                    let dir_content_clone: DirContent;

                    match dir_content {
                        DirContent::Dir(dir) => {
                            dir_content_clone = DirContent::Dir(dir.clone());
                            let dir_content_transmitter_clone = dir_content_transmitter.clone();
                            tokio::spawn(async move {
                                SimpleProducer::produce(DirContent::Dir(dir), pulse_transmitter, dir_content_transmitter_clone).await;
                            });
                        },
                        DirContent::File(file) => {
                            dir_content_clone = DirContent::File(file.clone());
                        } 
                    }
                    
                    let query_clone = self.query.clone();
                    tokio::spawn(async move {
                        LocalConsumer::consume(dir_content_clone, query_clone).await;
                    });

                },
                None=>{
                    break;
                }
            }
        }
    }
    
}