extern crate clap;
extern crate crossterm;
extern crate image;
extern crate base64;
#[macro_use]
extern crate itertools;
extern crate rayon;

#[macro_use]
mod library;
mod data;
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
use crate::{
	args::init_status,
	data::Args,
	draw_term::draw_term,
	draw_image::draw_image
};

fn main() {
	let args = Args::parse();
	let status = init_status(&args);
	match status.output {
		None    => draw_term(status),
		Some(_) => draw_image(status)
	}
}