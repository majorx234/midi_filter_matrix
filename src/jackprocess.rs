use bus::BusReader;
use crossbeam_channel::Receiver;
use jack;
use ringbuf::Producer;
use ringbuf::SharedRb;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::{process::exit, thread, time::Duration};

pub fn start_jack_thread(mut rx_close: BusReader<bool>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let (client, _status) =
            jack::Client::new("midi filter matrix", jack::ClientOptions::NO_START_SERVER)
                .expect("No Jack server running\n");
        let sample_rate = client.sample_rate();
        // register ports:
        let _midi_in = client.register_port("mfm_midi_in", jack::MidiIn).unwrap();
        let _midi_out = client.register_port("mfm_midi_out", jack::MidiOut).unwrap();
        let mut frame_size = client.buffer_size() as usize;
        if client.set_buffer_size(frame_size as u32).is_ok() {
            // get frame size
            let frame_size = client.buffer_size() as usize;
            println!(
                "client started with samplerate: {} and frame_size: {}",
                sample_rate, frame_size
            );
        } else {
            exit(-1);
        }
        if client.set_buffer_size(frame_size as u32).is_ok() {
            // get frame size
            frame_size = client.buffer_size() as usize;
            println!(
                "client started with samplerate: {} and frame_size: {}",
                sample_rate, frame_size
            );
        } else {
            exit(-1);
        }

        let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            jack::Control::Continue
        };
        let process = jack::ClosureProcessHandler::new(process_callback);
        let active_client = client.activate_async((), process).unwrap();
        while run {
            thread::sleep(Duration::from_millis(100));
            match rx_close.recv() {
                Ok(running) => run = running,
                Err(_) => run = false,
            }
        }
        match active_client.deactivate() {
            Ok(_) => println!("exit jackaudio thread\n"),
            Err(_) => println!("exit jackaudio thread,client deactivation err\n"),
        }
    })
}
