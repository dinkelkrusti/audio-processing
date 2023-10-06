use std::time::Duration;

use cpal::InputCallbackInfo;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use ringbuf::HeapRb;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    let input_device = host.default_input_device()
        .expect("Failed to find input");
    println!("Using input device: \"{}\"", input_device.name()?);


    let config: cpal::StreamConfig = input_device.default_input_config()?.into();

    let buffer = HeapRb::<f32>::new(1000);
    let (mut producer, mut consumer) = buffer.split();

    let write_into_buffer = move |data: &[f32], _: &InputCallbackInfo| for sample in data {
        println!("{:?}", &sample);
        match producer.push(*sample) {
            Ok(_) => {},
            Err(_) => { consumer.clear(); }
        };
    };

    let stream = input_device.build_input_stream(
        &config,
        write_into_buffer,
        err_fn,
        None,
    )?;

    stream.play()?;
    std::thread::sleep(Duration::from_secs(100));
    drop(stream);

    return Ok(());
}

fn err_fn(_: cpal::StreamError) {
    println!("REEEEEEE")
}