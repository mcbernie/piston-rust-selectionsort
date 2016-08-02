//use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::{ GlGraphics, OpenGL };
//use image::*;// as im;

use piston::input::{RenderArgs, UpdateArgs};
use glutin_window::GlutinWindow as Window;

use display::*;
//use texture::*;
use rand::{thread_rng, Rng, ThreadRng};

//use simplebar::SimpleBar;
use bargraph::BarGraph;

pub struct App {
    pub gl: GlGraphics,
    pub window: Window,
    bargraph: BarGraph,
    timercurrent: f64,
    rng: ThreadRng,
    unsorted_array: Vec<u32>,
    array_index: usize,
    sort_timer: f64,
}

impl App {

    pub fn new(opengl: OpenGL, window: Window) -> App {

        let gl = GlGraphics::new(opengl);
        let mut rng = thread_rng();

        // Generate Array with 350 unsorted items
        let mut v = Vec::new();
        for x in 0..350 {
            v.push(rng.gen_range(0, 100));
        }

        App {
            gl: gl,
            window: window,
            bargraph: BarGraph::new(350, 700, 200), // Set Graph for 350 items
            timercurrent: 0.0,
            rng: rng,
            unsorted_array: v,
            array_index: 0,
            sort_timer: 0.0,
        }

    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let bargraph = &self.bargraph;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(AppColors::MacBackground.into(), gl);

            bargraph.render(c.transform.trans(50.0, 0.0), c, gl);
        });

    }

    /// Simple Selectionsort Algorithm
    fn sortstep(&mut self) {
        //println!("next sortstep...");
        let un = &mut self.unsorted_array;
        if self.array_index + 1 >= un.len() {
            self.array_index = 0;
        }

        let current = un[self.array_index];
        let mut min_indx = self.array_index;

        for i in self.array_index..un.len() {
            if un[i] < un[min_indx] {
                min_indx = i
            }
        }

        un[self.array_index] = un[min_indx];
        un[min_indx] = current;

        self.array_index += 1;
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.timercurrent += args.dt;
        self.sort_timer += args.dt;

        if self.sort_timer > 0.02 {
            self.sortstep();

            self.sort_timer = 0.0;
        }



        if self.timercurrent >= 0.02 {
            let un = &self.unsorted_array;
            let targ:Color = AppColors::MacBackground.into();

            let bargraph = &mut self.bargraph.graph;

            let mut c:usize = 0;
            for i in un {
                let b = &mut bargraph[c];
                if b.current != *i {
                    b.update(*i,AppColors::Orange.into());
                } else {
                    let ccolor = b.color;
                    let mut newcolor:Color = [0.0,0.0,0.0,1.0];
                    let a = args.dt as f32 * 1.70;
                    newcolor[0] = (a * targ[0]).abs() + ((1.0 - a) * ccolor[0]);
                    newcolor[1] = (a * targ[1]).abs() + ((1.0 - a) * ccolor[1]);
                    newcolor[2] = (a * targ[2]).abs() + ((1.0 - a) * ccolor[2]);
                    b.update(*i,newcolor);
                }
                c += 1;
            }

            self.timercurrent = 0.0;
        }
    }

}
