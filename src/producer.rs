use crate::defs::{DirContent, DirContentSender, PulseSender, PULSE};

pub struct SimpleProducer {}

impl SimpleProducer {

    pub async fn produce(current_dir: DirContent, pulse_sender: PulseSender, dir_content_sender: DirContentSender) {
        let mut handles = Vec::new();

        let current_dir = match current_dir {
            DirContent::File(path)=>{path},
            DirContent::Dir(path)=>{path}
        };

        match current_dir.read_dir() {
            Ok(contents)=>{
                for file in contents {
                    match file {
                        Ok(file)=>{
                            let pulse_sender_clone = pulse_sender.clone();
                            let dir_content_sender_clone = dir_content_sender.clone();

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
        futures::future::join_all(handles).await;
    }
}
