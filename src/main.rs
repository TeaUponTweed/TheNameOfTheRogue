extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate specs;

use piston::window::WindowSettings;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL};
use specs

#[derive(Clone, Debug)]
struct Position(f32, f32);
impl specs::Component for Position {
    type Storage = specs::VecStorage<Position>;
}

#[derive(Clone, Debug)]
struct Velocity(f32, f32);
impl specs::Component for Velocity {
    type Storage = specs::VecStorage<Velocity>;
}
fn main() {
    let mut planner = {
        let mut w = specs::World::new();
        w.register::<Position>();
        w.register::<Velocity>();
        w.create_now().with(Position(3.0, 4.0)).build();
        w.create_now().with(Position(3.0, 4.0)).build();
        w.create_now().with(Position(0.0, 7.0)).with(Velocity(1.0, 1.0)).build();
        specs::Planner::<()>::new(w, 2)
    };
    loop {
        planner.run0w1r(|pos: &Position| {
            println!("Entity x {} y {}", pos.0, pos.1);
        });
        planner.run1w1r(|pos: &mut Position, vel: &Velocity| {
            pos.0 += vel.0;
            pos.1 += vel.1;
        });
        planner.wait();
    }
}

struct Renderer {
    gl: GlGraphics, // OpenGL drawing backend.
    win: Window,
}

impl Renderer {
    fn new() -> Renderer {
        let opengl = OpenGL::V3_2;
        // Create an Glutin window.
        let window: Window = WindowSettings::new(
                "spinning-square",
                [200, 200]
            ).opengl(opengl)
             .exit_on_esc(true)
             .build()
             .unwrap();
        let gl = GlGraphics::new(opengl);
        Renderer{gl: gl, win: window}
    }
}

impl specs::System<super::Delta> for Renderer {
    fn run(&mut self, arg: specs::RunArg, _: super::Delta) {
         for event in self.win.poll_events() {
            match(event) {
                // process events here
                _ => ()
            }
        }
    }
}