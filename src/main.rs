extern crate specs;
extern crate ncurses;

use std::{thread, time};

struct Pos(f32, f32);
impl specs::Component for Pos {
    type Storage = specs::VecStorage<Pos>;
}

struct Vel(f32, f32);
impl specs::Component for Vel {
    type Storage = specs::VecStorage<Vel>;
}

struct Disp(char);
impl specs::Component for Disp {
    type Storage = specs::VecStorage<Disp>;
}

pub fn main() {
    ncurses::initscr();

    let mut planner : specs::Planner<()> = {
        let mut w = specs::World::new();
        // All components types should be registered before working with them
        w.register::<Pos>();
        w.register::<Vel>();
        w.register::<Disp>();
        // create_now() of World provides with an EntityBuilder to add components to an Entity
        w.create_now().with(Pos(1.0, 6.0)).with(Vel(1.0, 2.0)).with(Disp('a')).build();
        w.create_now().with(Pos(2.0, 5.0)).with(Vel(3.0, 4.0)).with(Disp('b')).build();
        w.create_now().with(Pos(3.0, 4.0)).with(Disp('c')).build();
        // Planner is used to run systems on the specified world with a specified number of threads
        specs::Planner::new(w, 4)
    };
    loop {
        ncurses::erase();
        planner.run1w1r(|p: &mut Pos, v: &Vel| {
            p.0 += v.0;
            p.1 += v.1;
            if p.0 > 100.0 {
                p.0 = 0.0;
            }
            if p.1 > 100.0 {
                p.1 = 0.0;
            }
        });
        planner.run0w2r(|p: &Pos, c: &Disp| {
            ncurses::mvaddch(p.1.floor() as i32, p.0.floor() as i32, c.0 as u64);
        });
        planner.wait();
        ncurses::refresh();
        thread::sleep(time::Duration::from_millis(200));
    }
}
