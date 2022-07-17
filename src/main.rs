extern crate clap;
extern crate crossterm;
extern crate image;
extern crate base64;
#[macro_use]
extern crate itertools;
extern crate rayon;

#[macro_use]
mod lib;
mod args;
mod draw_term;
mod draw_image;
mod draw_lib;
mod draw_func;
mod draw_func_color;
mod draw_func_colorbar;
mod draw_func_mandelbrot;
mod draw_func_newton;
mod draw_func_ansi;

use clap::Parser;
use crate::args::{Args,init_status};
use crate::draw_term::draw_term;
use crate::draw_image::draw_image;

fn main() {
	let args = Args::parse();
	let status = init_status(&args);
	match status.output {
		None    => draw_term(status),
		Some(_) => draw_image(status)
	}
}