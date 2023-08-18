use argh::FromArgs;
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, PrintDiagnosticsPlugin},
    input::keyboard::{ElementState, KeyboardInput},
    input::mouse::{MouseButtonInput, MouseMotion},
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
    render::pass::ClearColor,
    window::CursorMoved,
};
use bigbang::{Entity, GravTree};
use rand::Rng;

fn main() {
    println!("Hello, world!");
}
