use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, Perlin};

#[derive(Default)]
pub struct Terrain {}

impl Terrain {
    pub fn generate(&mut self) {
        let fbm = Fbm::<Perlin>::new(0);

        PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(1024 , 1024)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build()
            .write_to_file("fbm.png");
    }
}