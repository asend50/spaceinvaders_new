use crate::ui::still_image::StillImage;
use macroquad::prelude::*;

pub struct Bullet {
    pub image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
  
}
impl Bullet {
    pub async fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        image_name: &str,
        stretch: bool,
        zoom: f32,
        speed: f32,

    ) -> Self {
        let image = StillImage::new(image_name, width, height, x, y, stretch, zoom).await;

        Bullet {
            image,
            speed,
            move_dir: vec2(0.0, 0.0),
        }
    }

    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

    pub fn shoot(&mut self) {
            self.move_dir = vec2(0.0, -3.0);

        if self.move_dir.length() > 0.0 {
            self.image.set_y(self.image.get_y() + self.move_dir.y * self.speed * get_frame_time() * 60.0);
        }
    }
   
}

