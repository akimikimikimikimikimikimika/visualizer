use crate::lib::*;
use crate::draw_func_color::color_fragment;
use crate::draw_func_colorbar::colorbar_fragment;
use crate::draw_func_mandelbrot::mandelbrot_fragment;
use crate::draw_func_newton::newton_fragment;
use crate::draw_func_ansi::*;
pub use rayon::prelude::*;

pub fn fragment(coord:CF,size:&CU,status:&Status) -> C {

	match &status.draw_mode {
		DM::Color(cs) => {
			color_fragment(&coord,&size,&cs)
		},
		DM::Colorbar(cbs) => {
			colorbar_fragment(&coord,&size,&cbs)
		},
		DM::Mandelbrot => {
			mandelbrot_fragment(&coord,&size)
		},
		DM::NewtonApprox(nas) => {
			newton_fragment(&coord,&size, &nas)
		},
		DM::Ansi => {
			ansi_fragment(&coord,&size)
		}
	}

}

pub fn unify_coord(x:u16,y:u16,size:&CU,subpixels:&[CF]) -> Vec<CF> {

	let x0 = (x as f64) / (size.0 as f64);
	let y0 = (y as f64) / (size.1 as f64);

	return subpixels.iter().map( |(x,y)| (x0+x,y0+y) ).collect();

}

pub fn get_color(x:u16,y:u16,size:&CU,s:&Status,subpixels:&[CF]) -> C {
	let coords = unify_coord(x,y,size,subpixels);
	match s.aa {
		0 => fragment(coords[0],size,s),
		_ => {
			let ct = coords
				.into_iter().par_bridge()
				.map( |coord| {
					let c = fragment(coord,size,s);
					match c {
						C::Float {r,g,b,a} => {
							let aas = subpixels.len() as f64;
							( r/aas , g/aas , b/aas , a/aas )
						},
						C::GFloat {v,a} => {
							let aas = subpixels.len() as f64;
							( v/aas , v/aas , v/aas , a/aas )
						},
						_ => { panic!(); }
					}
				} )
				.reduce(
					|| (0.0,0.0,0.0,0.0),
					|c1,c2| {
						( c1.0+c2.0 , c1.1+c2.1 , c1.2+c2.2 , c1.3+c2.3 )
					}
				);
			C::Float { r:ct.0, g:ct.1, b:ct.2, a:ct.3 }
		}
	}
}

pub fn aa_subpixels(aa:u8,size:&CU,stretched:bool) -> Vec<CF> {

	let     dx = 1.0 / (size.0 as f64) / (aa.max(1) as f64);
	let mut dy = 1.0 / (size.1 as f64) / (aa.max(1) as f64);

	if stretched && aa==0 { dy *= 2.0; }

	let ax = aa.max(1) as u16;
	let ay = match (stretched,aa) {
		(true ,0) => 1,
		(true ,_) => (aa*2) as u16,
		(false,_) => aa.max(1) as u16
	};

	return iproduct!(0..ax,0..ay)
		.map(|(i,j)| {
			( (i as f64)*dx , (j as f64)*dy )
		}).collect();

}