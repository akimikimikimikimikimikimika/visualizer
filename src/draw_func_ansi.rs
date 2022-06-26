use crate::draw_lib::*;

pub fn ansi_fragment(c:&CF,s:&CU) -> C {

	let wide = s.0>=(s.1*2);
	let (gn,gc) = match wide {
		false => (
			(c.1*5.0) as u8,
			(c.0,(c.1*5.0).rem_euclid(1.0))
		),
		true  => {
			let (x,y) = ((c.0*2.0) as u8,(c.1*3.0) as u8);
			if y==0 { (
				0,
				(c.0,(c.1*3.0).rem_euclid(1.0))
			) }
			else { (
				2*x+(y-1)+1,
				((c.0*2.0).rem_euclid(1.0),(c.1*3.0).rem_euclid(1.0))
			) }
		}
	};

	if gn==0 {
		let (cn,cc) = match gc.0<=(1.0/8.0) {
			true  => ((0,(gc.1*2.0) as u8),0.0),
			false => ((1,(gc.1*2.0) as u8),8.0/7.0*(gc.0-1.0/8.0))
		};
		match cn {
			(0,0) => C::None,
			(0,1) => C::Reverse,
			(1,0) => C::Ansi(
				match (cc*25.0).round() as u8 {
					0 => 0x10,
					v if v>0 && v<25 => v+0xe7,
					25 => 0xe7,
					_ => { panic!(); }
				}
			),
			(1,1) => C::GFloat{v:cc,a:1.0},
			_ => { panic!(); }
		}
	}
	else {
		let cn = ((gc.0*8.0) as u8,(gc.1*2.0) as u8);
		match gn {
			1 => {
				match cn {
					(0,0) => C::KL,
					(0,1) => C::KD,
					(1,0) => C::RL,
					(1,1) => C::RD,
					(2,0) => C::GL,
					(2,1) => C::GD,
					(3,0) => C::YL,
					(3,1) => C::YD,
					(4,0) => C::BL,
					(4,1) => C::BD,
					(5,0) => C::ML,
					(5,1) => C::MD,
					(6,0) => C::CL,
					(6,1) => C::CD,
					(7,0) => C::WL,
					(7,1) => C::WD,
					_ => { panic!(); }
				}
			},
			2 => C::Ansi(cn.0+8*(1-cn.1)),
			3 => {
				match cn {
					(0,0) => C::Int{r:0x80,g:0x80,b:0x80,a:0xff},
					(0,1) => C::Int{r:0x00,g:0x00,b:0x00,a:0xff},
					(1,0) => C::Int{r:0xff,g:0x00,b:0x00,a:0xff},
					(1,1) => C::Int{r:0x80,g:0x00,b:0x00,a:0xff},
					(2,0) => C::Int{r:0x00,g:0xff,b:0x00,a:0xff},
					(2,1) => C::Int{r:0x00,g:0x80,b:0x00,a:0xff},
					(3,0) => C::Int{r:0xff,g:0xff,b:0x00,a:0xff},
					(3,1) => C::Int{r:0x80,g:0x80,b:0x00,a:0xff},
					(4,0) => C::Int{r:0x00,g:0x00,b:0xff,a:0xff},
					(4,1) => C::Int{r:0x00,g:0x00,b:0x80,a:0xff},
					(5,0) => C::Int{r:0xff,g:0x00,b:0xff,a:0xff},
					(5,1) => C::Int{r:0x80,g:0x00,b:0x80,a:0xff},
					(6,0) => C::Int{r:0x00,g:0xff,b:0xff,a:0xff},
					(6,1) => C::Int{r:0x00,g:0x80,b:0x80,a:0xff},
					(7,0) => C::Int{r:0xff,g:0xff,b:0xff,a:0xff},
					(7,1) => C::Int{r:0xc0,g:0xc0,b:0xc0,a:0xff},
					_ => { panic!(); }
				}
			},
			4 => {
				C::Ansi(
					match cn {
						(0,0) => 244,
						(0,1) =>  16,
						(7,0) => 231,
						(7,1) => 250,
						_     => {
							let (r,g,b) = match cn {
								(1,0) => (5,0,0),
								(1,1) => (2,0,0),
								(2,0) => (0,5,0),
								(2,1) => (0,2,0),
								(3,0) => (5,5,0),
								(3,1) => (2,2,0),
								(4,0) => (0,0,5),
								(4,1) => (0,0,2),
								(5,0) => (5,0,5),
								(5,1) => (2,0,2),
								(6,0) => (0,5,5),
								(6,1) => (0,2,2),
								_ => { panic!(); }
							};
							r*36+g*6+b+0x10
						}
					}
				)
			},
			_ => { panic!(); }
		}
	}
}