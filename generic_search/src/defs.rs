use tonic::Result;
use tokio::sync::mpsc;
use std::path::PathBuf;

use crate::search::SearchResponse;

// Defines the content of a directory entry.
pub enum DirContent {
    File(PathBuf),
    Dir(PathBuf)
}

// Defines a pulse, as long as the pulse channel is open, the program will continue to run.
pub type Pulse = ();

// Defines the dummy pulse value.
pub static PULSE: () = ();

// Defines the sender for a pulse.
pub type PulseSender = mpsc::Sender<Pulse>;

// Defines the sender for the output stream.
pub type StreamSender = mpsc::Sender<Result<SearchResponse>>;

// Defines the sender for the output stream.
pub type StreamReciever = mpsc::Receiver<Result<SearchResponse>>;

// Defines the message to be sent over the channel.
pub type DirContentChannelMessage = (Vec<DirContent>, PulseSender);

// Defines the sender for a directory entry.
pub type DirContentSender = mpsc::Sender<DirContentChannelMessage>;
