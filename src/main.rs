use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use cpal::InputCallbackInfo;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, circle_boi)
        // .add_systems(Startup, listen_to_audio_in)
        .run()
}

fn circle_boi(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
}

fn listen_to_audio_in() {
    fn err_fn(_: cpal::StreamError) {
        println!("REEEEEEE")
    }

    let host = cpal::default_host();

    let input_device = host.default_input_device()
        .expect("Failed to find input");
    println!("Using input device: \"{}\"", input_device.name().unwrap());


    let config: cpal::StreamConfig = input_device.default_input_config().unwrap().into();

    let write_into_buffer = move |data: &[f32], _: &InputCallbackInfo| for sample in data {
        println!("{:?}", &sample);
    };

    let stream = input_device.build_input_stream(
        &config,
        write_into_buffer,
        err_fn,
        None,
    ).unwrap();

    stream.play().unwrap();

    std::thread::sleep(Duration::from_secs(1));
    drop(stream);
}