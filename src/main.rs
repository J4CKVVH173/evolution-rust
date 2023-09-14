mod cell;
mod map;
mod world;

use std::{thread::sleep, time::Duration};

use self::world::World;

fn main() {
    let mut world = World::default();
    world.populate_world();

    loop {
        world.run();
        sleep(Duration::from_millis(500));
        let m = world.get_map();
        println!();
        for row in m {
            let mut row_str = String::new();
            for cell in row {
                match cell {
                    Some(v) => {
                        if v.kind == "Herbivore" {
                            row_str.push('h');
                        } else if v.kind == "Organic" {
                            row_str.push('x');
                        } else {
                            row_str.push('p');
                        }
                    },
                    None => row_str.push('0'),
                }
                row_str.push(' ');
            }
            println!("{}", &row_str);
        }
        println!();
    }
}
