use std::ops::Deref;
use std::process::exit;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread::available_parallelism, num::NonZeroUsize, collections::VecDeque};

use colored::*;
use clap::Parser;
use threadpool::ThreadPool;

type SafeQueuePtr = Arc<Mutex<VecDeque<PathBuf>>>;

#[derive(Parser, Debug)]
#[command(author, version, about = "Find your zombie!", long_about = None)]
pub struct Args {
    /// The directory to search in.
    #[arg(index = 1, short, long)]
    dir: String,

    /// The query (file/dir name) to search for.
    #[arg(index = 2, short, long)]
    query: String,

    /// Ignore case when searching.
    #[arg(long, default_value = "false")]
    ignore_case: bool,

    /// Only print files, do not use with only_dirs!.
    #[arg(long, default_value = "false")]
    only_files: bool,

    /// Only print directories, do not use with only_files!.
    #[arg(long, default_value = "false")]
    only_dirs: bool,
}

fn process_file_name(path: &PathBuf, query: &Box<String>, args: &Arc<Args>) {
    let is_match = match args.ignore_case {
        true => path.file_name().unwrap().to_ascii_lowercase().to_str().unwrap().contains(query.to_ascii_lowercase().deref()),
        false => path.file_name().unwrap().to_str().unwrap().contains(query.deref()),
    };
        
    if (path.is_file() || path.is_dir()) && is_match {
        let string = path.to_str().unwrap().to_string().green();
        let is_dir = path.is_dir();
        match is_dir {
            true => {
                if args.only_files {
                    return;
                }
                println!("{} {}", "[DIR] ".green().blink(), string.blink());
            },
            false => {
                if args.only_dirs {
                    return;
                }
                println!("{} {}", "[FIL] ".green(), string);
            },
        }
    }
}

pub fn run(root_dir: &String, query: &String, args: Arc<Args>) {
    let val = Arc::new(AtomicUsize::new(0));
    let path = PathBuf::from(root_dir);
    let query = Box::new(query.clone());
    if !path.is_dir() {
        panic!("{} is not a directory", path.display());
    }
    let num_threads = available_parallelism()
                                    .unwrap_or(NonZeroUsize::new(4).unwrap())
                                    .get();
    println!("{} {} {}", "Using".cyan().bold(), num_threads.to_string().cyan().bold(), "threads".cyan().bold());
    let worker_pool = ThreadPool::new(num_threads);

    let dir_queue = VecDeque::from(vec![path]);
    let dir_queue = Arc::new(Mutex::new(dir_queue));
    loop {
        let safeq = dir_queue.lock().unwrap();
        if !safeq.is_empty() {
            let val = Arc::clone(&val);
            let dir_queue_clone = dir_queue.clone();
            let thread_query = query.clone();
            let thread_args = args.clone();
            worker_pool.execute(move || {
                val.fetch_add(1, Ordering::SeqCst);
                search_single_dir(dir_queue_clone, thread_query, thread_args);
                val.fetch_sub(1, Ordering::SeqCst);
            });
        } else if val.load(Ordering::SeqCst) == 0 {            
            break;
        }
    }
}

fn search_single_dir(dir_queue: SafeQueuePtr, query: Box<String>, args: Arc<Args>) {
    let system_path: Option<PathBuf>;
    {
        let mut safeq = dir_queue.lock().unwrap();
        system_path = safeq.pop_front();
    }
    let system_path = match system_path {
        Some(path) => path,
        None => return,
    };
    let entries = match system_path.read_dir() {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in  entries {
        let entry = entry.unwrap();
        let path = entry.path();
        process_file_name(&path, &query, &args);
        if path.is_dir() {
            {
                let mut safeq = dir_queue.lock().unwrap();
                safeq.push_back(path);
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let root_dir = args.dir.clone();
    let query = args.query.clone();

    if args.only_dirs && args.only_files {
        println!("{}", "You can't use both --only-files and --only-dirs".red());
        exit(1);
    }
    run(&root_dir, &query, Arc::new(args));
}