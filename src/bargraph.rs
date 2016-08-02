use opengl_graphics::{ GlGraphics, Texture, OpenGL };
use simplebar::SimpleBar;
use graphics::math::Matrix2d;
use graphics::Transformed;
use graphics::Graphics;
use graphics::image::Image as im;
use graphics::context::Context;

use image::*;
use display::*;
use texture::*;

pub struct BarGraph {
    pub graph: Vec<SimpleBar>,
    width: u32,
    height: u32,
    el_width: f64,
}

impl BarGraph {
    pub fn new(bars:u32, width: u32, height: u32) -> BarGraph {

        let el_width = width / bars;
        let mut vec = Vec::new();

        for nel in 0..(bars) {
            //println!("Init: {}", nel);
            vec.push(SimpleBar::new(el_width, height));
        }

        BarGraph {
            width: width,
            height: height,
            el_width: el_width as f64,
            graph: vec,
        }
    }

    pub fn render(&self,t:Matrix2d, c:Context, gl:&mut GlGraphics) {
        let mut x:f64 = 0.0;
        let el_width:f64 = self.el_width;
        //let m = t.trans(-( self.width as f64 / 2.0),  -( self.height as f64 / 2.0) );

        for bar in &self.graph {
            let img = im::new();
            img.draw(&bar.texture, &c.draw_state, t.trans(x, 0.0), gl);
            x+= el_width;
        }
    }
}
