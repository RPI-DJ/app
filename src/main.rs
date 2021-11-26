use jack::MidiIn;
use std::thread;
use std::sync::mpsc::{sync_channel, channel};
use std::io;

fn main() {
    let (sender, reciver) = sync_channel(64);
    let (signal_sender, signal_reciver) = channel();
    midi_handler::midi_init(sender, signal_reciver);
    thread::spawn(move || {
        while let Ok(m) = reciver.recv() {
            println!("{:?}", m);
        }
    });

    println!("Press any key to quit");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();

    signal_sender.send(1).unwrap();
}
