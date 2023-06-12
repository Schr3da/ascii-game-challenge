use image::imageops::blur;
use image::Rgba;
use imageproc::drawing::draw_filled_ellipse;
use imageproc::filter::gaussian_blur_f32;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Fbm, Perlin};

#[derive(Default)]
pub struct Terrain {}

impl Terrain {
    pub fn generate(&mut self) {
        let fbm = Fbm::<Perlin>::new(0);

        PlaneMapBuilder::<_, 2>::new(&fbm)
            .set_size(512, 512)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build()
            .write_to_file("fbm-1.png");

        let img = image::open("./example_images/fbm-1.png").unwrap();
        let circle = draw_filled_ellipse(&img, (256, 256), 200, 200, Rgba([0, 0, 0, 255]));
        circle.save("./example_images/fbm-2.png").unwrap();
        

        blur(&img, 35.0).save("./example_images/fbm-3.png").unwrap();
        gaussian_blur_f32(&img.to_rgb8(), 35.0)
            .save("./example_images/fbm-4.png")
            .unwrap();
    }
}
