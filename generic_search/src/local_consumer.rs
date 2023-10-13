use crate::defs::DirContent;
use crate::common::{ print_local_file_match, print_local_directory_match };

pub struct LocalConsumer {}

impl LocalConsumer {
    
    // Consume the directory content. Prints out the file/dir name if it contains the query.
    pub async fn consume(dir_content_vec: Vec<DirContent>, query: String) {
        
        for dir_content in dir_content_vec {

            let path = match dir_content {
                DirContent::File(path)=>{path},
                DirContent::Dir(path)=>{path}
            };
    
            if path.to_str().unwrap().to_lowercase().contains(query.to_lowercase().as_str()) {
                if path.is_file() {
                    // Print the file name in green.
                    print_local_file_match(path.to_str().unwrap());
                } else if path.is_dir() {
                    // Print the directory name in green and blink.
                    print_local_directory_match(path.to_str().unwrap());
                }
            }
        }
    }
}