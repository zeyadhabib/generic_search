use colored::*;
use crate::defs::DirContent;

pub struct LocalConsumer {}

impl LocalConsumer {
    
    pub async fn consume(dir_content: DirContent, query: String) {
        let path = match dir_content {
            DirContent::File(path)=>{path},
            DirContent::Dir(path)=>{path}
        };

        if path.to_str().unwrap().to_lowercase().contains(query.to_lowercase().as_str()) {
            if path.is_file() {
                println!("{} {}", "[FIL] ".green(), path.to_str().unwrap().to_string().green());
            } else if path.is_dir() {
                println!("{} {}", "[DIR] ".green().blink(), path.to_str().unwrap().to_string().green().blink());
            }
        }
    }
}