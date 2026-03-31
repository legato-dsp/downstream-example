use std::fs;

use cpal::{
    SampleRate, StreamConfig,
    traits::{DeviceTrait, HostTrait},
};
use legato::{
    builder::{LegatoBuilder, Unconfigured},
    config::{BlockSize, Config},
    midi::{MidiPortKind, start_midi_thread},
    out::start_application_audio_thread,
    ports::PortBuilder,
};

fn main() {
    let graph = fs::read_to_string("../.legato").expect("Could not fine legato file!");

    let config = Config::new(48_000, BlockSize::Block256, 2, 6);

    let ports = PortBuilder::default().audio_out(2).build();

    let (midi_rt_fe, _writer_fe) = start_midi_thread(
        256,
        "my_port",
        MidiPortKind::Index(0),
        MidiPortKind::Index(0),
        "my_port",
    )
    .unwrap();

    let (app, _frontend) = LegatoBuilder::<Unconfigured>::new(config, ports)
        .set_midi_runtime(midi_rt_fe)
        .build_dsl(&graph);

    #[cfg(target_os = "macos")]
    let host = cpal::host_from_id(cpal::HostId::CoreAudio).expect("JACK host not available");

    #[cfg(target_os = "linux")]
    let host = cpal::host_from_id(cpal::HostId::Jack).expect("JACK host not available");

    let devices = host.output_devices().expect("Failed to get output devices");

    println!("Available output devices:");
    for device in devices {
        // 3. Print device name
        if let Ok(name) = device.name() {
            println!("  {}", name);
        }
    }

    let device = host.default_output_device().unwrap();

    dbg!(device.name());

    let stream_config = StreamConfig {
        channels: config.channels as u16,
        sample_rate: SampleRate(config.sample_rate as u32),
        buffer_size: cpal::BufferSize::Fixed(config.block_size as u32),
    };

    // std::thread::spawn(move || {
    //     std::thread::sleep(Duration::from_secs(5));
    //     frontend.set_param("pitch", 880.0).unwrap();
    // });

    start_application_audio_thread(&device, stream_config, app).expect("Audio thread panic!");
}
