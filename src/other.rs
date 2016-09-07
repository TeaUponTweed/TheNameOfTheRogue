extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL};
use graphics::rectangle::square;

#[derive(Debug)]
struct Entity {
    x: i32,
    y: i32,
}

static TILE_WIDTH: u8 = 8;
fn main() {
    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        ).opengl(opengl)
         .exit_on_esc(true)
         .build()
         .unwrap();
    let mut gl = GlGraphics::new(opengl);
    let mut entities = Vec::new();
    let mut cursor = [0.0, 0.0];
    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        e.mouse_cursor(|x, y| {
            cursor = [x, y];
        });
        use piston::input::mouse::MouseButton;
        if let Some(Button::Mouse(button)) = e.press_args() {
            match button {
                 MouseButton::Right => {entities.push( Entity { x: cursor[0].floor() as i32, 
                                                                 y: cursor[1].floor() as i32});},
                b                   => {println!("Pressed mouse button '{:?}'", b);},
            }
        }

        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, gl| {
                use graphics::*;

                clear([0.0, 0.0, 0.0, 1.0], gl);
                let square = rectangle::square(0.0, 0.0, TILE_WIDTH as f64);

                const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
                for &Entity{x, y} in &entities {
                    let transform = c.transform.trans(x as f64, y as f64)
                                     .trans(-(TILE_WIDTH as f64)/2.0, -(TILE_WIDTH as f64)/2.0);

                    rectangle(RED, square, transform, gl);
                }
            });
        }
    }
}
