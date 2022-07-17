use crate::draw_lib::*;

pub fn colorbar_fragment(c:&CF,s:&CU,cbs:&ColorbarStatus) -> C {

	let p_opt = rationalize_coord(
		c,&s,
		match cbs.mode {
			CBSMode::SMPTE => ( 4,3),
			CBSMode::ARIB  => (16,9)
		},
		match cbs.repeat {
			false => RCOverflow::Discard,
			true  => RCOverflow::Repeat
		},
		RCOrigin::TopLeft
	);
	if p_opt==None { return C::None }
	let p = p_opt.unwrap();

	match cbs.mode {
		CBSMode::SMPTE => smpte_fragment(p),
		CBSMode::ARIB  =>  arib_fragment(p)
	}

}

fn smpte_fragment(p:CF) -> C {

	// mapped position
	let mp:(u8,u8) = match (p.1*12.0) as u8 {
		0..=7 => {
			let xn = (p.0*7.0) as u8;
			(xn,0)
		},
		8 => {
			let xn = (p.0*7.0) as u8;
			(xn,1)
		},
		9..=12 => {
			let xs = p.0*7.0;
			let xn = match xs as u8 {
				0..=4 => (xs*4.0/5.0) as u8,
				5     => ((xs-5.0)*3.0) as u8 + 4,
				6     => 7,
				_     => { return C::None }
			};
			(xn,2)
		},
		_ => { return C::None }
	};

	match mp {
		(0,0)|(6,1)       => C::Int{r:204,g:204,b:204,a:255},
		(1,0)             => C::Int{r:255,g:255,b:  0,a:255},
		(2,0)|(4,1)       => C::Int{r:  0,g:255,b:255,a:255},
		(3,0)             => C::Int{r:  0,g:255,b:  1,a:255},
		(4,0)|(2,1)       => C::Int{r:255,g:  0,b:254,a:255},
		(5,0)             => C::Int{r:254,g:  0,b:  0,a:255},
		(6,0)|(0,1)       => C::Int{r:  0,g:  0,b:254,a:255},
		(1,1)|(3,1)|(5,1) => C::Int{r: 19,g: 19,b: 19,a:255},
		(0,2)             => C::Int{r:  8,g: 62,b: 90,a:255},
		(1,2)             => C::Int{r:255,g:255,b:255,a:255},
		(2,2)             => C::Int{r: 59,g:  0,b:126,a:255},
		(4,2)             => C::Int{r:  0,g:  0,b:  0,a:255},
		(3,2)|(5,2)|(7,2) => C::Int{r: 19,g: 19,b: 19,a:255},
		(6,2)             => C::Int{r: 38,g: 38,b: 38,a:255},
		_ => C::None
	}

}

fn arib_fragment(p:CF) -> C {

	// mapped position
	let mut mp:(u8,u8,f64) = (
		match p.0 {
			x if x> 0.0      && x<=(1.0/8.0) => 0,
			x if x< 1.0      && x>=(7.0/8.0) => 1,
			x if x>(1.0/8.0) && x< (7.0/8.0) => 2,
			_ => { return C::None }
		},
		match (p.1*12.0) as u8 {
			0..=6  => 0,
			7      => 1,
			8      => 2,
			9..=12 => 3,
			_      => { return C::None }
		},
		0.0
	);
	if mp.0==2 {
		mp.2 = 8.0/6.0*(p.0-1.0/8.0);
		match mp.1 {
			0 => { mp.0 += (mp.2*7.0) as u8 },
			1 => {
				if mp.2>=(1.0/7.0) { mp.0 = 3 }
			},
			3 => {
				// n等分していきながら分岐
				let xs = mp.2*7.0;
				mp.0 = match xs as u8 {
					0|1|2|3 => {
						match (xs*2.0) as u8 {
							0|1|2   => 2,
							3|4|5|6 => 3,
							7       => 4,
							_       => { return C::None }
						}
					},
					4|5 => {
						((xs-4.0)*3.0) as u8 + 4
					},
					6 => 10,
					_ => { return C::None }
				};
			},
			2 => {},
			_ => { return C::None }
		}
	}

	match mp {
		// 両端
		(0|1,0,_)       => C::Float{r:0.4 ,g:0.4 ,b:0.4 ,a:1.0},
		(0  ,1,_)       => C::Float{r:0.0 ,g:1.0 ,b:1.0 ,a:1.0},
		(1  ,1,_)       => C::Float{r:0.0 ,g:0.0 ,b:1.0 ,a:1.0},
		(0  ,2,_)       => C::Float{r:1.0 ,g:1.0 ,b:0.0 ,a:1.0},
		(1  ,2,_)       => C::Float{r:1.0 ,g:0.0 ,b:0.0 ,a:1.0},
		(0|1,3,_)       => C::Float{r:0.15,g:0.15,b:0.15,a:1.0},
		// 中央
		(2,0,_)|(3,1,_) => C::Float{r:0.75,g:0.75,b:0.75,a:1.0},
		(3,0,_)         => C::Float{r:0.75,g:0.75,b:0.0 ,a:1.0},
		(4,0,_)         => C::Float{r:0.0 ,g:0.75,b:0.75,a:1.0},
		(5,0,_)         => C::Float{r:0.0 ,g:0.75,b:0.0 ,a:1.0},
		(6,0,_)         => C::Float{r:0.75,g:0.0 ,b:0.75,a:1.0},
		(7,0,_)         => C::Float{r:0.75,g:0.0 ,b:0.0 ,a:1.0},
		(8,0,_)         => C::Float{r:0.0 ,g:0.0 ,b:0.75,a:1.0},
		(2,1,_)         => C::Float{r:1.0 ,g:1.0 ,b:1.0 ,a:1.0},
		(2,2,d)         => C::GFloat{v:d,a:1.0},
		(2|4|8|10,3,_)  => C::Float{r:0.0 ,g:0.0 ,b:0.0 ,a:1.0},
		(3       ,3,_)  => C::Float{r:1.0 ,g:1.0 ,b:1.0 ,a:1.0},
		(5       ,3,_)  => C::Float{r:0.0 ,g:0.0 ,b:0.0 ,a:1.0}, // actually -2 %
		(6       ,3,_)  => C::Float{r:0.0 ,g:0.0 ,b:0.0 ,a:1.0},
		(7       ,3,_)  => C::Float{r:0.02,g:0.02,b:0.02,a:1.0},
		(9       ,3,_)  => C::Float{r:0.04,g:0.04,b:0.04,a:1.0},
		_ => C::None
	}

}