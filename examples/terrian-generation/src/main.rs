use core_terrain::prelude::*;

fn main() {
    let mut terrain = Terrain::default();
    terrain.generate();

    _ = terrain.get_value(400, 400);
}
