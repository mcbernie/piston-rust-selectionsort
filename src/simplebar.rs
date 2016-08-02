use opengl_graphics::{ Texture };
use texture::*;
use image::*;

pub struct SimpleBar {
    width: u32,
    height: u32,
    image: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub texture: Texture,
    pub current: u32,
    pub color: [f32; 4],
}

impl SimpleBar {

    pub fn new(width:u32, max_height:u32) -> SimpleBar {

        //let imload = open("assets/test.png").unwrap().to_rgba();

        let imload =  ImageBuffer::new(width, max_height);

        //println!("testimage: {}", testimage);
        let texture = Texture::from_image(
            &imload,
            &TextureSettings::new()
        );

        SimpleBar {
            width: width,
            height: max_height,
            image: imload,
            texture: texture,
            current: 0,
            color: [0.0,0.0,0.0,1.0]
        }
    }

    fn update_texture(&mut self, height:u32, color: [f32; 4]) {
        self.color = color;
        let white = self.height - height;
        for x in 0..self.width {
            for y in 0..self.height {
                if y <= white {
                    //draw hidden pixel with alpha 0
                    self.image.put_pixel(x,y,Rgba([0, 0, 0, 0]));
                } else {
                    self.image.put_pixel(x,y,Rgba([
                        (self.color[0] * 255.0) as u8,
                        (self.color[1] * 255.0) as u8,
                        (self.color[2] * 255.0) as u8,
                        (self.color[3] * 255.0) as u8
                        ])
                    );
                }
            }
        }

        self.texture.update(&self.image);
    }

    pub fn update(&mut self, percent:u32, color: [f32; 4]) {
        self.current = percent;
        let height = (percent * self.height) / 100;
        self.update_texture(height, color);
    }

}
