use crate::defs::{ DirContent, StreamSender };
use crate::search::SearchResponse;

pub struct RemoteConsumer {}

impl RemoteConsumer {
    
    // Consume the directory content. Prints out the file/dir name if it contains the query.
    pub async fn consume(stream_sender: StreamSender, dir_content_vec: Vec<DirContent>, query: String) {
        
        for dir_content in dir_content_vec {

            let path = match dir_content {
                DirContent::File(path)=>{path},
                DirContent::Dir(path)=>{path}
            };
    
            if path.to_str().unwrap().to_lowercase().contains(query.to_lowercase().as_str()) {
                if path.is_file() {
                    // Print the file name in green.
                    let search_response = SearchResponse{
                        r#match: path.to_str().unwrap().to_string(),
                        is_directory: false
                    };
                    let _ = stream_sender.send(Ok(search_response)).await;
                } else if path.is_dir() {
                    // Print the directory name in green and blink.
                    let search_response = SearchResponse{
                        r#match: path.to_str().unwrap().to_string(),
                        is_directory: true
                    };
                    let _ = stream_sender.send(Ok(search_response)).await;
                }
            }
        }
    }
}