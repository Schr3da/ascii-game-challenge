use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{Curve, Fbm, MultiFractal, Perlin};

use core_dtos::prelude::*;

#[derive(Default)]
pub struct Terrain {
    pub land: NoiseMap,
    pub vegetation: NoiseMap,
    pub mountains: NoiseMap,
}

const SEA_LEVEL: f64 = 0.0;

pub static MAP_SIZE: usize = 256;

impl Terrain {
    fn generate_land(&mut self) {
        let data = Fbm::<Perlin>::new(8)
            .set_frequency(0.2)
            .set_persistence(0.6)
            .set_lacunarity(2.1)
            .set_octaves(5);

        let base_limits = Curve::new(data.clone())
            .add_control_point(-2.0000 + SEA_LEVEL, -1.625 + SEA_LEVEL)
            .add_control_point(-1.3000 + SEA_LEVEL, -1.175 + SEA_LEVEL)
            .add_control_point(0.0000 + SEA_LEVEL, -0.375 + SEA_LEVEL)
            .add_control_point(0.0625 + SEA_LEVEL, 0.125 + SEA_LEVEL)
            .add_control_point(0.1250 + SEA_LEVEL, 1.5 + SEA_LEVEL)
            .add_control_point(0.2500 + SEA_LEVEL, 1.5 + SEA_LEVEL);

        self.land = PlaneMapBuilder::<_, 2>::new(&base_limits)
            .set_size(MAP_SIZE, MAP_SIZE)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();
    }

    pub fn generate_vegetation(&mut self) {
        let data = Fbm::<Perlin>::new(8)
            .set_frequency(0.2)
            .set_persistence(0.6)
            .set_lacunarity(2.5)
            .set_octaves(10);

        let base_limits = Curve::new(data.clone())
            .add_control_point(1.0 + SEA_LEVEL, 0.0)
            .add_control_point(1.05 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(1.1 + SEA_LEVEL, 0.0 + SEA_LEVEL)
            .add_control_point(1.2 + SEA_LEVEL, 1.5 + SEA_LEVEL)
            .add_control_point(1.3 + SEA_LEVEL, 0.0 + SEA_LEVEL)
            .add_control_point(1.4 + SEA_LEVEL, 1.5 + SEA_LEVEL)
            .add_control_point(2.0 + SEA_LEVEL, 1.500 + SEA_LEVEL);

        self.vegetation = PlaneMapBuilder::<_, 2>::new(&base_limits)
            .set_size(MAP_SIZE, MAP_SIZE)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();
    }

    pub fn generate_mountains(&mut self) {
        let data = Fbm::<Perlin>::new(8)
            .set_frequency(0.2)
            .set_persistence(0.6)
            .set_lacunarity(2.1)
            .set_octaves(5);

        let mountain_limits = Curve::new(data)
            .add_control_point(1.0 + SEA_LEVEL, 0.0)
            .add_control_point(1.05 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(1.4 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(1.5 + SEA_LEVEL, 1.500 + SEA_LEVEL)
            .add_control_point(2.0 + SEA_LEVEL, 1.500 + SEA_LEVEL);

        self.mountains = PlaneMapBuilder::<_, 2>::new(&mountain_limits)
            .set_size(MAP_SIZE, MAP_SIZE)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();
    }

    pub fn generate(&mut self) {
        self.generate_land();
        self.generate_vegetation();
        self.generate_mountains();
    }

    pub fn get_value(&self, x: i32, y: i32) -> f64 {
        let land_value = self.land.get_value(x as usize, y as usize);
        let mountain_value = self.mountains.get_value(x as usize, y as usize);

        if mountain_value > 0.0 {
            return mountain_value;
        }

        return land_value;
    }

    fn contains_value(current: f64, start: f64, end: f64) -> bool {
        current >= start && current < end
    }

    pub fn value_to_ascii(&self, value: f64) -> AsciiIds {
        if Self::contains_value(value, -5.0, -0.5) {
            return AsciiIds::DeepWater;
        }

        if Self::contains_value(value, -0.5, 0.1) {
            return AsciiIds::ShallowWater;
        }

        if Self::contains_value(value, 0.1, 2.5) {
            return AsciiIds::Sand;
        }

        return AsciiIds::UnknownAsciiId;
    }

    pub fn get_ascii(&self, x: i32, y: i32) -> AsciiIds {
        let next = self.get_value(x, y);
        self.value_to_ascii(next)
    }

    pub fn save_as_images(&self) {
        self.land.write_to_file("land.png");
        self.vegetation.write_to_file("vegetation.png");
        self.mountains.write_to_file("mountain.png");
    }
}
