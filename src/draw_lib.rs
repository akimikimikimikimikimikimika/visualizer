pub use crate::lib::*;
use std::f64::consts::PI;

extern crate num;
pub use num::{Complex,complex::ComplexFloat};

pub type Z = Complex<f64>;

pub const RAD2DEG:f64 = 180.0/PI;

pub fn rationalize_coord(coord:&CF,size:&CU,ratio:CU,overflow:RCOverflow,origin:RCOrigin) -> Option<CF> {
	let sf:CF = ( size.0 as f64, size.1 as f64);
	let rf:CF = (ratio.0 as f64,ratio.1 as f64);
	let cpa:CF = ( // container size / actual size
		sf.0.min(sf.1/rf.1*rf.0) / sf.0,
		sf.1.min(sf.0/rf.0*rf.1) / sf.1
	);

	match origin {
		RCOrigin::TopLeft => {
			let norm:CF = (
				(coord.0-0.5)/cpa.0+0.5,
				(coord.1-0.5)/cpa.1+0.5,
			);
			match overflow {
				RCOverflow::Keep => Some(norm),
				RCOverflow::Repeat => {
					Some((
						norm.0.rem_euclid(1.0),
						norm.1.rem_euclid(1.0)
					))
				},
				RCOverflow::Discard => {
					if norm.0<0.0 || norm.0>1.0 || norm.1<0.0 || norm.1>1.0 { None }
					else { Some(norm) }
				}
			}
		},
		RCOrigin::Center => {
			let mut norm:CF = (
				(coord.0*2.0-1.0)/ cpa.0,
				(coord.1*2.0-1.0)/-cpa.1,
			);
			match overflow {
				RCOverflow::Keep => Some(norm),
				RCOverflow::Repeat => {
					norm = (
						norm.0.rem_euclid(2.0),
						norm.1.rem_euclid(2.0)
					);
					if norm.0>1.0 { norm.0 = norm.0-2.0 }
					if norm.1>1.0 { norm.1 = norm.1-2.0 }
					Some(norm)
				},
				RCOverflow::Discard => {
					if norm.0.abs()>1.0 || norm.1.abs()>1.0 { None }
					else { Some(norm) }
				}
			}
		}
	}
}

pub enum RCOverflow {
	Keep,
	Repeat,
	Discard
}

pub enum RCOrigin {
	Center,
	TopLeft
}

pub fn deg2rgb(deg:f64) -> (f64,f64,f64) {
	let can = deg.rem_euclid(360.0);
	let norm = can.rem_euclid(60.0)/60.0;
	(
		match can {
			t if ( t >  240.0 && t <  300.0 ) =>     norm,
			t if ( t >= 300.0 || t <=  60.0 ) => 1.0     ,
			t if ( t >   60.0 && t <  120.0 ) => 1.0-norm,
			_ => 0.0
		},
		match can {
			t if ( t >    0.0 && t <   60.0 ) =>     norm,
			t if ( t >=  60.0 && t <= 180.0 ) => 1.0     ,
			t if ( t >  180.0 && t <  240.0 ) => 1.0-norm,
			_ => 0.0
		},
		match can {
			t if ( t>  120.0 && t<  180.0 ) =>     norm,
			t if ( t>= 180.0 && t<= 300.0 ) => 1.0     ,
			t if ( t>  300.0 && t<  360.0 ) => 1.0-norm,
			_ => 0.0
		}
	)
}