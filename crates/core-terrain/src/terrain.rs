use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{Curve, Fbm, MultiFractal, Perlin };

#[derive(Default)]
pub struct Terrain {
    pub land: NoiseMap,
    pub mountains: NoiseMap,
}

const SEA_LEVEL: f64 = 0.0;

impl Terrain {
    pub fn generate(&mut self) {
        let data = Fbm::<Perlin>::new(8)
            .set_frequency(0.2)
            .set_persistence(0.5)
            .set_lacunarity(2.2)
            .set_octaves(12);

        let base_limits = Curve::new(data.clone())
            .add_control_point(-2.0000 + SEA_LEVEL, -1.625 + SEA_LEVEL)
            .add_control_point(-1.0000 + SEA_LEVEL, -1.375 + SEA_LEVEL)
            .add_control_point(0.0000 + SEA_LEVEL, -0.375 + SEA_LEVEL)
            .add_control_point(0.0625 + SEA_LEVEL, 0.125 + SEA_LEVEL)
            .add_control_point(0.1250 + SEA_LEVEL, 1.5 + SEA_LEVEL)
            .add_control_point(0.2500 + SEA_LEVEL, 1.5 + SEA_LEVEL);

        self.land = PlaneMapBuilder::<_, 2>::new(&base_limits)
            .set_size(512, 512)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();

        let mountain_limits = Curve::new(data)
            .add_control_point(1.2 + SEA_LEVEL, 0.0)
            .add_control_point(1.3 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(1.4 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(1.5 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(2.0 + SEA_LEVEL, 1.500 + SEA_LEVEL);

        self.mountains = PlaneMapBuilder::<_, 2>::new(&mountain_limits)
            .set_size(512, 512)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();
    }

    pub fn save_as_images(&self) {
        self.land.write_to_file("land.png");
        self.mountains.write_to_file("mountain.png");
    }

    pub fn get_value(&self, x: usize, y: usize) -> i32 {
        let land_value = self.land.get_value(x, y) as i32;
        let mountain_value = self.mountains.get_value(x, y) as i32;

        if mountain_value > 0 {
            println!("Mountain Value: {}", mountain_value);
            return mountain_value;
        }

        println!("Land Value: {}", land_value);
        return land_value;
    }
}
