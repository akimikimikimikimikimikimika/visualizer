use crate::draw_lib::*;

pub fn mandelbrot_fragment(c:&CF,s:&CU) -> C {

	let p = rationalize_coord(c,&s,(1,1),RCOverflow::Keep,RCOrigin::Center).unwrap();

	let c = Z { re: p.0-0.5, im: p.1 };

	let mut z = Z{ re:0.0, im:0.0 };
	for n in 0..200 {
		z = z.powi(2) + c;
		if z.abs() > 2.0 {
			let rgb = deg2rgb(240.0-(n as f64)/200.0*180.0);
			return C::Float{r:rgb.0*0.6,g:rgb.1*0.6,b:rgb.2*0.6,a:1.0}
		}
	}

	C::GFloat{v:0.0,a:1.0}

}