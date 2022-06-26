use crate::lib::*;
use crate::draw_func_color::color_fragment;
use crate::draw_func_colorbar::colorbar_fragment;
use crate::draw_func_mandelbrot::mandelbrot_fragment;
use crate::draw_func_ansi::*;
pub use rayon::prelude::*;

pub fn fragment(coord:CF,size:&CU,status:&Status) -> C {

	match &status.draw_mode {
		DM::Color(cs)     => color_fragment(&coord,&size,&cs),
		DM::Colorbar(cbs) => colorbar_fragment(&coord,&size,&cbs),
		DM::Mandelbrot    => mandelbrot_fragment(&coord,&size),
		DM::Ansi          => ansi_fragment(&coord,&size)
	}

}

pub fn unify_coord(x:u16,y:u16,size:&CU) -> CF {
	(
		( (x as f64) + 0.5 )/(size.0 as f64),
		( (y as f64) + 0.5 )/(size.1 as f64)
	)
}

pub fn unify_coord_stretched(x:u16,y:u16,size:&CU) -> CF {
	(
		( (x as f64) + 0.5 )/(size.0 as f64),
		( (y as f64) + 1.0 )/(size.1 as f64)
	)
}