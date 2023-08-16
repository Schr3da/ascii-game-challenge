use noise::utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder};
use noise::{Curve, Fbm, MultiFractal, Perlin};

use core_dtos::prelude::*;

#[derive(Default)]
pub struct Terrain {
    pub land: NoiseMap,
    pub vegetation: NoiseMap,
    pub mountains: NoiseMap,
    pub visibility: NoiseMap,
    pub buildings: NoiseMap,
}

const SEA_LEVEL: f64 = 0.0;

const PERLIN_SEED: u32 = 12;

pub static MAP_SIZE: usize = 128;

impl Terrain {
    fn generate_land(&mut self) {
        let data = Fbm::<Perlin>::new(PERLIN_SEED)
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
        let data = Fbm::<Perlin>::new(PERLIN_SEED)
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
        let data = Fbm::<Perlin>::new(PERLIN_SEED)
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

    fn generate_visibility(&mut self) {
        let (width, height) = self.land.size();
        self.visibility = NoiseMap::new(width, height);
    }

    fn generate_initial_buildings(&mut self) {
        let (width, height) = self.land.size();
        self.buildings = NoiseMap::new(width, height);

        self.buildings.set_value(0, 1, AsciiIds::HeadQuarter.to_float());
        self.visibility.set_value(0, 1, AsciiIds::Visible.to_float());
    }

    pub fn generate(&mut self) {
        self.generate_land();
        self.generate_vegetation();
        self.generate_mountains();
        self.generate_visibility();
        self.generate_initial_buildings();
    }

    pub fn get_value(&self, x: i32, y: i32) -> f64 {
        let visibility = self.visibility.get_value(x as usize, y as usize);
        if visibility == AsciiIds::NotVisible.to_float(){
            return -100.0;
        }

        let building_value = self.buildings.get_value(x as usize, y as usize);
        if building_value > AsciiIds::UnknownAsciiId.to_float() {
            return AsciiIds::HeadQuarter.to_float();
        }

        let mountain_value = self.mountains.get_value(x as usize, y as usize);
        if mountain_value > AsciiIds::UnknownAsciiId.to_float() {
            return mountain_value;
        }

        return self.land.get_value(x as usize, y as usize);
    }

    pub fn get_ascii(&self, x: i32, y: i32) -> AsciiIds {
        let next = self.get_value(x, y);
        AsciiIds::value_to_id(next)
    }

    pub fn save_as_images(&self) {
        self.land.write_to_file("land.png");
        self.vegetation.write_to_file("vegetation.png");
        self.mountains.write_to_file("mountain.png");
        self.visibility.write_to_file("visibility.png");
        self.buildings.write_to_file("buildings.png");
    }
}
