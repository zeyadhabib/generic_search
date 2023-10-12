use crate::defs::{DirContent, DirContentSender, PulseSender, PULSE};

pub struct SimpleProducer {}

impl SimpleProducer {

    pub async fn produce(current_dir: DirContent, pulse_sender: PulseSender, dir_content_sender: DirContentSender) {
        let mut files = Vec::new();

        // Unwrap the current directory.
        let current_dir = match current_dir {
            // File match is not really need since this is guarantted to be a directory.
            DirContent::File(path)=>{path},
            DirContent::Dir(path)=>{path}
        };

        match current_dir.read_dir() {
            Ok(contents)=>{

                // Clone the pulse transmitter and directory content transmitter for the thread.
                let pulse_sender_clone = pulse_sender.clone();
                let dir_content_sender_clone = dir_content_sender.clone();

                for file in contents {
                    match file {
                        Ok(file)=>{
                            
                            let file = match file.path().is_dir()  {
                                true => {DirContent::Dir(file.path())},
                                false => {DirContent::File(file.path())}
                            };

                            files.push(file);
                        },
                        Err(_)=>{}
                    }
                }

                // Spawn a thread to send the directory content and pulse transmitter over the channel, to the orchestrator.
                let _ = pulse_sender_clone.send(PULSE).await;
                let _ = dir_content_sender_clone.send((files, pulse_sender_clone)).await;
            },
            Err(_)=>{}
        }
    }
}
