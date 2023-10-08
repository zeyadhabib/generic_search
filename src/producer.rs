use crate::defs::{DirContent, DirContentSender, PulseSender, PULSE};

pub struct SimpleProducer {}

impl SimpleProducer {

    pub async fn produce(current_dir: DirContent, pulse_sender: PulseSender, dir_content_sender: DirContentSender) {
        let mut handles = Vec::new();

        // Unwrap the current directory.
        let current_dir = match current_dir {
            // File match is not really need since this is guarantted to be a directory.
            DirContent::File(path)=>{path},
            DirContent::Dir(path)=>{path}
        };

        match current_dir.read_dir() {
            Ok(contents)=>{
                for file in contents {
                    match file {
                        Ok(file)=>{
                            // Clone the pulse transmitter and directory content transmitter for the thread.
                            let pulse_sender_clone = pulse_sender.clone();
                            let dir_content_sender_clone = dir_content_sender.clone();

                            // Spawn a thread to send the directory content and pulse transmitter over the channel, to the orchestrator.
                            let handle = tokio::spawn(
                                async move {
                                    let file = match file.path().is_dir()  {
                                        true => {DirContent::Dir(file.path())},
                                        false => {DirContent::File(file.path())}
                                    };

                                    let _ = pulse_sender_clone.send(PULSE).await;
                                    let _ = dir_content_sender_clone.send((file, pulse_sender_clone)).await;
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
        // Join on all the spawned threads.
        futures::future::join_all(handles).await;
    }
}
