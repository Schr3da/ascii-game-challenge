use core_terrain::prelude::*;

fn main() {
    let mut terrain = Terrain::default();
    terrain.generate();
    terrain.save_as_images();

    let value = terrain.get_value(167, 130);
    println!("Position Value {value}");
}
