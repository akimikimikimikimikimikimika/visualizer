use crate::draw_lib::*;

pub fn color_fragment(c:&CF,s:&CU,cs:&ColorStatus) -> C {
	match cs.mode {
		CSMode::Conic|CSMode::ConicW => hue_fragment(c,s,cs),
		CSMode::BV|CSMode::BVW => bvw_fragment(c,cs)
	}
}

pub fn color_mouse_down(c:&CF,s:&CU,cs:&mut ColorStatus) {
	match cs.mode {
		CSMode::Conic|CSMode::ConicW => hue_mouse_down(c,s,cs),
		CSMode::BV|CSMode::BVW => bvw_mouse_down(c,cs)
	}
}

pub fn color_mouse_drag(c:&CF,s:&CU,cs:&mut ColorStatus) {
	match cs.mode {
		CSMode::Conic|CSMode::ConicW => hue_mouse_drag(c,s,cs),
		CSMode::BV|CSMode::BVW => bvw_mouse_drag(c,cs)
	}
}

fn hue_fragment(c:&CF,s:&CU,cs:&ColorStatus) -> C {

	let p = rationalize_coord(c,&s,(1,1),RCOverflow::Keep,RCOrigin::Center).unwrap();
	let     radius = p.1.hypot(p.0);
	let mut theta  = p.1.atan2(p.0)*RAD2DEG;
	theta = (theta-cs.angle).rem_euclid(360.0);

	match radius {
		d if d>0.0 => {
			let (r,g,b) = deg2rgb(theta);
			let d_norm:f64 = match cs.mode {
				CSMode::ConicW => d.min(1.0),
				CSMode::Conic  => 1.0,
				_ => { return C::None; }
			};
			C::Float{
				r:(1.0-d_norm*(1.0-r)),
				g:(1.0-d_norm*(1.0-g)),
				b:(1.0-d_norm*(1.0-b)),
				a:1.0
			}
		},
		d if d<=0.0 => C::Float{r:1.0,g:1.0,b:1.0,a:1.0},
		_ => C::None
	}

}

fn hue_mouse_down(c:&CF,s:&CU,cs:&mut ColorStatus) {
	cs.mouse_position = rationalize_coord(c,s,(1,1),RCOverflow::Keep,RCOrigin::Center).unwrap();
}

fn hue_mouse_drag(c:&CF,s:&CU,cs:&mut ColorStatus) {

	let mpp = cs.mouse_position;
	let mpc = rationalize_coord(c,s,(1,1),RCOverflow::Keep,RCOrigin::Center).unwrap();
	cs.mouse_position = mpc;
	let angle_diff = (mpc.1*mpp.0-mpc.0*mpp.1).atan2(mpc.0*mpp.0+mpc.1*mpp.1)*RAD2DEG;
	cs.angle = (cs.angle+angle_diff).rem_euclid(360.0);

}

fn bvw_fragment(c:&CF,cs:&ColorStatus) -> C {
	let theta = (c.0*360.0+cs.angle).rem_euclid(360.0);
	let hue = deg2rgb(theta);

	match cs.mode {
		CSMode::BVW => {
			if c.1<0.5 {
				C::Float{
					r:1.0-2.0*c.1*(1.0-hue.0),
					g:1.0-2.0*c.1*(1.0-hue.1),
					b:1.0-2.0*c.1*(1.0-hue.2),
					a:1.0
				}
			}
			else {
				C::Float{
					r:2.0*(1.0-c.1)*hue.0,
					g:2.0*(1.0-c.1)*hue.1,
					b:2.0*(1.0-c.1)*hue.2,
					a:1.0
				}
			}
		},
		CSMode::BV => {
			C::Float{
				r:(1.0-c.1)*hue.0,
				g:(1.0-c.1)*hue.1,
				b:(1.0-c.1)*hue.2,
				a:1.0
			}
		},
		_ => C::None
	}

}

fn bvw_mouse_down(c:&CF,cs:&mut ColorStatus) {
	cs.mouse_position = (c.0,c.1);
}

fn bvw_mouse_drag(c:&CF,cs:&mut ColorStatus) {

	let mpp = cs.mouse_position;
	let mpc = (c.0,c.1);
	cs.mouse_position = mpc;
	let angle_diff = (mpc.0-mpp.0)*360.0;
	cs.angle = (cs.angle-angle_diff).rem_euclid(360.0);

}