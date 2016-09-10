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
// struct Entity {
//     x: i32,
//     y: i32,
// }
enum Compontent {
    Position(f64, f64),
    Velocity(f64, f64),
    Size(f64),
}

#[derive(Debug)]
struct Entity {
    components: Vec<Compontent>,
}

// e.components.filter_map(|c| {
//     match c {
//         Compontent::Position(x, y) => Compontent::Position(x, y),
//         _ => None,
//     }
// })

static TILE_WIDTH: u8 = 8;
// pub struct App {
//     gl: GlGraphics, // OpenGL drawing backend.
//     enities: Vec<Entity>,
// }

// impl App {
//     fn render(&mut self, args: &RenderArgs) {
//         use graphics::*;

//         const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
//         const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

//         let square = rectangle::square(0.0, 0.0, 50.0);

//         self.gl.draw(args.viewport(), |c, gl| {
//             // Clear the screen.
//             clear(GREEN, gl);
//             for entity in self.entities {
//                 let transform = c.transform.trans(entity.x, entity.y)
//                                            .trans(-25.0, -25.0);
//                 rectangle(RED, square, transform, gl);
//             }
//         });
//     }

//     fn new() -> App P
// }

fn main() {
    use Compontent::*;
    let mut e = Entity{ components: vec![Position(1.0, 3.0), Velocity(4.0, 6.0), Size(10.0), Position(12.3, 45.5)]};

    let iter = e.components.iter().filter_map(|c| {
        match *c {
            Position(x, y) => Some(Position(x, y)),
            _ => None,
        }
    });
    for el in iter {
        println!("{:?}", el);
    }

    /*
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
    */
}
