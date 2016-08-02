extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate texture;
extern crate rand;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ OpenGL };

mod display;
mod app;
mod simplebar;
mod bargraph;


#[cfg(any(windows))]
fn set_opengl() -> OpenGL {
    OpenGL::V3_1
}

#[cfg(not(windows))]
fn set_opengl() -> OpenGL {
    OpenGL::V3_3
}


fn main() {

    let opengl = set_opengl();

    let window: Window = WindowSettings::new(
            "Test Algorithm",
            [800,300]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = app::App::new(opengl,window);
    let mut events = app.window.events();

    while let Some(e) = events.next(&mut app.window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }

}
