// https://github.com/PistonDevelopers/image
extern crate image;
extern crate rand;
extern crate specs;

use std::fs::File;
use std::path::Path;
use rand::distributions::{IndependentSample, Range};
use std::cmp::{Ordering, Ord};

#[derive(Debug)]
#[derive(Eq)]
struct RegionPoint {
    x: i32,
    y: i32,
    color : image::Rgb<u8>,
}

impl Ord for RegionPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.x.cmp(&other.x)
    }
}

impl PartialOrd for RegionPoint {
    fn partial_cmp(&self, other: &RegionPoint) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RegionPoint {
    fn eq(&self, other: &RegionPoint) -> bool {
        self.x == other.x
    }
}

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
/*
#[derive(Debug)]
enum Component {
    Position{x: f32, y: f32},
    Velocity{dx: f32, dy: f32},
}

#[derive(Debug)]
struct Enity {
    components: Vec<Component>,
}
*/
// #[cfg(feature="parallel")]

/*
fn main() {
    let imgx = 4000;
    let imgy = 4000;

    let mut rng = rand::thread_rng();
    let xrange = Range::new(0, imgx);
    let yrange = Range::new(0, imgy);
    let mut points =  Vec::new();
    for _ in 0..14 {
        let x = xrange.ind_sample(&mut rng);
        let y = yrange.ind_sample(&mut rng);
        let (r, g, b) = rand::random::<(u8, u8, u8)>();
        let color = image::Rgb([r, g, b]);
        points.push(RegionPoint{x: x, y: y, color: color});
    }
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let wakka = points.iter().map(|p| {
            ((p.x - x as i32).abs() + (p.y - y as i32).abs(), p)
        }).min();
        match  wakka {
            Some((_, closest_point)) => {*pixel = closest_point.color},
            None => {*pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8])},
        }
    }
    let ref mut fout = File::create(&Path::new("map.png")).unwrap();
    let _ = image::DynamicImage::ImageRgb8(imgbuf).save(fout, image::PNG);
}
*/
/*
#[derive(Debug)]
enum Biome { // 3x3 matrix, random walk between them (need characteristic spites[s] too)
    Swamp(color: image::Rgb<u8>),
    Jungle(color: image::Rgb<u8>),
    Fjords(color: image::Rgb<u8>),
    Plains(color: image::Rgb<u8>),
    Forest(color: image::Rgb<u8>),
    Alpine(color: image::Rgb<u8>),
    Desert(color: image::Rgb<u8>),
    Steppe(color: image::Rgb<u8>),
    Tundra(color: image::Rgb<u8>),
}
*/

/*
#[derive(Debug)]
struct Biome {
    color: image::Rgb<u8>,
}

const BIOMES: [Biome; 9] = [Swamp {color: image::Rgb([25  as u8, 51  as u8, 0   as u8])},
                            Jungle{color: image::Rgb([0   as u8, 204 as u8, 0   as u8])},
                            Fjords{color: image::Rgb([153 as u8, 153 as u8, 255 as u8])},
                            Plains{color: image::Rgb([178 as u8, 255 as u8, 102 as u8])},
                            Forest{color: image::Rgb([0   as u8, 102 as u8, 0   as u8])},
                            Alpine{color: image::Rgb([102 as u8, 255 as u8, 178 as u8])},
                            Desert{color: image::Rgb([255 as u8, 255 as u8, 204 as u8])},
                            Steppe{color: image::Rgb([255 as u8, 204 as u8, 255 as u8])},
                            Tundra{color: image::Rgb([204 as u8, 255 as u8, 255 as u8])}];
*/
/*
want representation of user input.
enum Input {

}
*/
/*
binding of momentum
binding of translation

*/
