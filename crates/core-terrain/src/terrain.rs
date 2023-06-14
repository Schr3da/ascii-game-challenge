use image::imageops::blur;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Curve, Fbm, MultiFractal, Perlin};

#[derive(Default)]
pub struct Terrain {}

const SEA_LEVEL: f64 = 0.0;

impl Terrain {
    pub fn generate(&mut self) {
        let base = Fbm::<Perlin>::new(8)
            .set_frequency(0.2)
            .set_persistence(0.5)
            .set_lacunarity(2.2)
            .set_octaves(12);

        let land = Curve::new(base.clone())
            .add_control_point(-2.0000 + SEA_LEVEL, -1.625 + SEA_LEVEL)
            .add_control_point(-1.0000 + SEA_LEVEL, -1.375 + SEA_LEVEL)
            .add_control_point(0.0000 + SEA_LEVEL, -0.375 + SEA_LEVEL)
            .add_control_point(0.0625 + SEA_LEVEL, 0.125 + SEA_LEVEL)
            .add_control_point(0.1250 + SEA_LEVEL, 0.250 + SEA_LEVEL)
            .add_control_point(0.2500 + SEA_LEVEL, 1.000 + SEA_LEVEL)
            .add_control_point(0.5000 + SEA_LEVEL, 0.250 + SEA_LEVEL)
            .add_control_point(0.7500 + SEA_LEVEL, 0.250 + SEA_LEVEL)
            .add_control_point(1.0000 + SEA_LEVEL, 0.500 + SEA_LEVEL)
            .add_control_point(2.0000 + SEA_LEVEL, 0.500 + SEA_LEVEL);

        PlaneMapBuilder::<_, 2>::new(&land)
            .set_size(512, 512)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build()
            .write_to_file("land-1.png");

        let mountain = Curve::new(base)
            .add_control_point(1.2 + SEA_LEVEL, 0.1 + SEA_LEVEL)
            .add_control_point(1.3 + SEA_LEVEL, 1.100 + SEA_LEVEL)
            .add_control_point(1.4 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(1.5 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(2.0 + SEA_LEVEL, 1.500 + SEA_LEVEL);

        PlaneMapBuilder::<_, 2>::new(&mountain)
            .set_size(512, 512)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build()
            .write_to_file("mountain-1.png");
    }
}
