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

#[derive(FromArgs)]
#[argh(description = "n-body sim in bevy w/ bigbang")]
struct Options {
    #[argh(option, default = "100", short = 'n', description = "number of bodies in the simulation")]
    num_bodies: usize,
    #[argh(option, default = "0.02", short = 't', description = "granularity of simulation (how much each frame impacts movement)")]
    time_step: f64,
    #[argh(option, default = "1280", short = 'w', description = "initial width of spawned window")]
    width: u32,
    #[argh(option, default = "720", short = 'h', description = "initial height of spawned window")]
    height: u32,
    #[argh(option, default = "10.0", short = 's', description = "initial scale of view (bigger = more zoomed out)")]
    scale: f32
}

struct Simulation(GravityTree<Entity>);

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    keyboard_event_reader: EventReader<KeyboardInput>,
    cursos_position: Option<Vec2>,
    zooming: bool,
    panning: bool,
    paused: bool,
    follow_body_index: Option<usize>,
}

fn main() {
    let options: Options = argh::from_env();
    //Bevy build
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "n-body".to_string(),
            width: options.width,
            height: options.height,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(PrintDiagnosticsPlugin::default())
        .init_resource::<State>()
        .add_resouce(ClearColor(Color::rgb(0.01, 0.01, 0.01)))
        .add_resource(Simulation(GravTree::new(
            &initialize_bodies(options.num_bodies, options.width, options.height, options.scale),
            options.time_step,
        )))
        .add_resource(options)
        .add_startup_system(add_bodies.system())
        .add_system(time_step.system())
        .add_system(update_bodies.system())
        .add_system(follow.system())
        .add_system(mouse_input.system())
        .add_system(keyboard_input.system())
        .run();
}

fn initialize_bodies(num: usize, width: u32, height: u32, scale: f32) -> Vec<Entity> {
    let mut rng = rand::thread_rng();
    let mut bodies = vec![];

    for i in 0..num {
        let mass = if i == 0 {
            rng.gen_range(500.,1500.)
        } else {
            rng.gen_range(50., 500.)
        };
        bodies.push(Entity {
            x: rng.gen_range(
                -(width as f64 / 2.) * scale as f64,
                (width as f64 / 2.) * scale as f64,
            ),
            y: rng.gen_range(
                -(height as f64 / 2.) * scale as f64,
                (height as f64 / 2.) * scale as f64,
            ),
            z: 0.,
            vx: rng.gen_range(-50., 50.),
            vy: rng.gen_range(-50., 50.),
            vz: 0.,
            mass,
            radius: mass / 30.,
        });
    }
    bodies
}

fn add_bodies() {
}
