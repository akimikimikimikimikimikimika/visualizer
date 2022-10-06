use crate::draw_lib::*;

pub fn newton_fragment(c:&CF,s:&CU,nas:&NewtonApproxStatus) -> C {

	let pf = nas.p as f64;

	let p = rationalize_coord(c,&s,(1,1),RCOverflow::Keep,RCOrigin::Center).unwrap();

	let mut z = Z { re:p.0, im:p.1 };

	for n in 0..nas.max {
		let dz = - nas.tau * (z.powi(nas.p as i32)-1.0) / (pf*z.powi((nas.p-1) as i32));
		if dz.abs()<=1e-6 {
			let rgb = deg2rgb(z.arg()*RAD2DEG);
			return match nas.speed {
				false => C::Float{r:rgb.0*0.6,g:rgb.1*0.6,b:rgb.2*0.6,a:1.0},
				true  => {
					let d = 1.0 - (n as f64) / (nas.max as f64);
					C::Float{r:rgb.0*d,g:rgb.1*d,b:rgb.2*d,a:1.0}
				}
			};
		}
		z += dz;
	}

	C::GFloat{v:0.0,a:1.0}

}