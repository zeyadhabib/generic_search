use tokio::sync::mpsc;
use std::path::PathBuf;

// Defines the content of a directory entry.
pub enum DirContent {
    File(PathBuf),
    Dir(PathBuf)
}

// Defines a pulse, as long as the pulse channel is open, the program will continue to run.
type Pulse = ();

pub static PULSE: () = ();

// Defines the sender for a pulse.
pub type PulseSender = mpsc::Sender<Pulse>;

type DirContentChannelMessage = (DirContent, PulseSender);

// Defines the sender for a directory entry.
pub type DirContentSender = mpsc::Sender<DirContentChannelMessage>;
