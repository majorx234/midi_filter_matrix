use bus::BusReader;
use crossbeam_channel::Receiver;
use jack;
use std::{process::exit, thread, time::Duration};
use struct_iterable::Iterable;

use crate::midi_matrix::MidiMatrix;
use crate::vector4b::Vector4b;

pub fn start_jack_thread(
    mut rx_close: BusReader<bool>,
    rx_midi_matrix_event: Receiver<MidiMatrix>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut run: bool = true;
        let (client, _status) =
            jack::Client::new("midi filter matrix", jack::ClientOptions::NO_START_SERVER)
                .expect("No Jack server running\n");
        let sample_rate = client.sample_rate();
        // register ports:
        let midi_in0 = client.register_port("mfm_midi_in0", jack::MidiIn).unwrap();
        let midi_in1 = client.register_port("mfm_midi_in1", jack::MidiIn).unwrap();
        let midi_in2 = client.register_port("mfm_midi_in2", jack::MidiIn).unwrap();
        let midi_in3 = client.register_port("mfm_midi_in3", jack::MidiIn).unwrap();
        let midi_in_vec = [midi_in0, midi_in1, midi_in2, midi_in3];
        let midi_out0 = client
            .register_port("mfm_midi_out0", jack::MidiOut)
            .unwrap();
        let midi_out1 = client
            .register_port("mfm_midi_out1", jack::MidiOut)
            .unwrap();
        let midi_out2 = client
            .register_port("mfm_midi_out2", jack::MidiOut)
            .unwrap();
        let midi_out3 = client
            .register_port("mfm_midi_out3", jack::MidiOut)
            .unwrap();
        let mut midi_out_vec = [midi_out0, midi_out1, midi_out2, midi_out3];
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

        let mut midi_mat = MidiMatrix::new();

        let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            // rx_midi_matrix_event
            if let Ok(rx_midi_matrix_event) = rx_midi_matrix_event.try_recv() {
                midi_mat = rx_midi_matrix_event;
            }
            for (midi_in_events, (_, midi_in_opt)) in midi_in_vec.iter().zip(midi_mat.iter()) {
                if let Some(matrix_midi_in) = midi_in_opt.downcast_ref::<Vector4b>() {
                    for e in midi_in_events.iter(ps) {
                        for ((_, matrix_midi_out_opt), midi_out) in
                            matrix_midi_in.iter().zip(midi_out_vec.iter_mut())
                        {
                            if let Some(matrix_midi_out) =
                                matrix_midi_out_opt.downcast_ref::<bool>()
                            {
                                if *matrix_midi_out {
                                    let _ = midi_out.writer(ps).write(&e);
                                }
                            }
                        }
                    }
                }
            }

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
