extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate vec_map;
extern crate clock_ticks;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL};
use graphics::rectangle::square;
#[macro_use] mod ecs;
use ecs::*;
use vec_map::VecMap;

#[derive(Copy, Clone, Debug)]
pub struct Pos(f64, f64);
#[derive(Copy, Clone, Debug)]
pub struct Vel(f64, f64);

world!{
    name: Game,
    components: {
        pos => Pos,
        vel => Vel
    },
    processors: {
        movement => Movement for [pos, vel]
    }
}
#[derive(Copy, Default, Clone, Debug)]
pub struct Movement;
impl Movement {
    fn run(&mut self, delta: f64, entities: &[Entity], pos: &mut Components<Pos>, vel: &Components<Vel>) {
        for &e in entities.iter() {
            let Vel(vx, vy) = vel[e];
            let &mut Pos(ref mut x, ref mut y) = &mut pos[e];
            *x += vx * delta;
            *y += vy * delta;
        }
    }
}


pub fn main() {
    let mut world:Game = World::new();
    let entity = world.create();
    let entity1 = world.create();
    // println!("{:?}", entity1);
    world.pos.insert(entity, Pos(1.0, -2.0));
    world.pos.insert(entity1, Pos(10.0, 10.0));
    world.vel.insert(entity, Vel(3.0, 2.0));
    world.vel.insert(entity1, Vel(30.0, 20.0));
    for _ in 0..10 {
        world.update(1.0);
        for (id, &el) in world.pos.iter() {
            // println!("{:?}", id);
            println!("{:?}\n", el);
        }
    }
}
