use std::ops::Deref;
use std::process::exit;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, mpsc};
use std::{thread::available_parallelism, num::NonZeroUsize};

use colored::*;
use clap::Parser;
use threadpool::ThreadPool;

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

    /// Ignore read file/dir errors.
    #[arg(long, short, default_value = "false")]
    ignore_errors: bool,
}

fn process_file_name(path: &PathBuf, query: &Arc<String>, args: &Arc<Args>) {
    // First off let's check if the file/dir name matches the query
    let is_match = match args.ignore_case {
        true => path.file_name().unwrap().to_ascii_lowercase().to_str().unwrap().contains(query.to_ascii_lowercase().deref()),
        false => path.file_name().unwrap().to_str().unwrap().contains(query.deref()),
    };
    
    // If it does, let's make sure that it's not a symlink.
    // We ignore symlinks to prevent cycles in the file system graph.
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

fn multi_threaded_dfs (root_dir: Arc<String>, query: Arc<String>, worker_pool: Arc<Mutex<ThreadPool>>, args: Arc<Args>, tx: Sender<()>) {
    let system_path = PathBuf::from(root_dir.deref());
    let entries = match system_path.read_dir() {
        Ok(entries) => entries,
        Err(err) => {
            if !args.ignore_errors {
                // Print the error and return, (most likely a permission error)
                println!("{} {} {} {}", "Error reading directory:".red(),
                            system_path.to_str().unwrap().red(), "err".red(), err.to_string().red())
            }
            return;
        },
    };

    for entry in  entries {
        let entry = entry.unwrap();
        let child_path = entry.path();
        process_file_name(&child_path, &query, &args);
        // If the child is a directory, let's spawn a new thread to search it.
        if child_path.is_dir() {
            // Clone the thread params to move into the new thread.
            let pool_clone = worker_pool.clone();
            let path_clone = Arc::new(child_path.to_str().unwrap().to_string());
            let query_clone = query.clone();
            let args_clone = args.clone();
            let tx_clone = tx.clone();
            {
                // Open a new scope to drop the lock on the worker pool.
                let pool_handle = worker_pool.lock().unwrap();
                pool_handle.execute(move || {
                    multi_threaded_dfs(path_clone, query_clone, pool_clone, args_clone, tx_clone);
                });
            }
        }
    }
}

fn main() {
    // TODO: Add comments
    let args = Args::parse();
    let root_dir = args.dir.clone();
    let query = args.query.clone();

    if args.only_dirs && args.only_files {
        println!("{}", "You can't use both --only-files and --only-dirs".red());
        exit(1);
    }

    let num_threads = available_parallelism()
                                    .unwrap_or(NonZeroUsize::new(4).unwrap())
                                    .get();
    println!("{} {} {}", "Using".cyan().bold(), num_threads.to_string().cyan().bold(), "threads".cyan().bold());
    let worker_pool = Arc::new(Mutex::new(ThreadPool::new(num_threads)));
    let (tx, rx) = mpsc::channel();
    multi_threaded_dfs(Arc::new(root_dir), Arc::new(query), worker_pool.clone(), Arc::new(args), tx);
    let _ = rx.recv();
}