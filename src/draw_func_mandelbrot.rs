use crate::draw_lib::*;

pub fn mandelbrot_fragment(c:&CF,s:&CU) -> C {

	let mut p = rationalize_coord(c,&s,(1,1),RCOverflow::Keep,RCOrigin::Center).unwrap();
	p.0 -= 0.5;

	let (mut a,mut b) = (0.0,0.0);
	for n in 0..200 {
		(a,b) = (a*a-b*b+p.0,2.0*a*b+p.1);
		if a.hypot(b)>2.0 {
			let rgb = deg2rgb(240.0-(n as f64)/200.0*180.0);
			return C::Float{r:rgb.0*0.6,g:rgb.1*0.6,b:rgb.2*0.6,a:1.0}
		}
	}

	C::Int{r:0,g:0,b:0,a:255}

}