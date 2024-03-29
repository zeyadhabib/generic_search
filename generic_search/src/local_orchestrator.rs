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

        // Create the channel for the pulse, this is created to prevent the program from hanging.
        let (pulse_transmitter, mut pulse_reciever) = mpsc::channel(32);
        // Create the channel for the directory content.
        let (dir_content_transmitter, mut dir_content_reciever) = mpsc::channel(32);

        // Clone the data needed by the producer and keep the original copys to spawn producers.
        let dir_content_transmitter_clone = dir_content_transmitter.clone();
        let root_dir = DirContent::Dir(self.root_dir.clone());

        // Spawn the first producer to get the ball rolling.
        tokio::spawn(async move {
            SimpleProducer::produce(root_dir, pulse_transmitter, dir_content_transmitter_clone).await;
        });

        loop {
            // As long as the pulse channel is open, the program will continue to run.
            let pulse_option = pulse_reciever.recv().await;
            match pulse_option {
                // Channel is open, continue to run.
                Some(_)=>{

                    // Recieve the directory content and pulse transmitter from the channel.
                    let (dir_content_vec, pulse_transmitter) = dir_content_reciever.recv().await.unwrap();
                    // Declare the clone of the directory content for the consumer thread.
                    let query = self.query.clone();
                    let dir_content_transmitter_internal = dir_content_transmitter.clone();

                    tokio::spawn(async move {
                        
                        let mut consumed_dir_content_vec: Vec<DirContent> = Vec::new();
                        for dir_content in dir_content_vec {
                            let dir_content_clone: DirContent;
                            match dir_content {
                                DirContent::Dir(dir) => {
                                    dir_content_clone = DirContent::Dir(dir.clone());
                                    let pulse_transmitter_clone = pulse_transmitter.clone();
                                    let dir_content_transmitter_clone = dir_content_transmitter_internal.clone();
    
                                    tokio::spawn(async move {
                                        SimpleProducer::produce(DirContent::Dir(dir), pulse_transmitter_clone, dir_content_transmitter_clone).await;
                                    });
                                },
                                DirContent::File(file) => {
                                    dir_content_clone = DirContent::File(file.clone());
                                }
                            }
                            consumed_dir_content_vec.push(dir_content_clone);
                        }

                        let query_clone = query.clone();
                        // Spawn the consumer thread.
                        tokio::spawn(async move {
                            LocalConsumer::consume(consumed_dir_content_vec, query_clone).await;
                        });
                    });
                },
                // Channel is closed, stop the program.
                None=>{
                    break;
                }
            }
        }
    }
    
}