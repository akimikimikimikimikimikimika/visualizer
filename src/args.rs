use crate::data::*;

/// コマンドラインパーサ (Args) から内部のデータ管理型 (Status) を生成する。
pub fn init_status(a:&Args) -> Status {

	let dm = match a.draw_mode {
		Some(EDM::Ansi)|None => DM::Ansi,
		Some(EDM::Hue {rotate:r}) => {
			let s = ColorStatus {
				angle: r,
				mode:  CSMode::ConicW,
				mouse_position: (0.0,0.0)
			};
			DM::Color(s)
		},
		Some(EDM::Conic {rotate:r}) => {
			let s = ColorStatus {
				angle: r,
				mode: CSMode::Conic,
				mouse_position: (0.0,0.0)
			};
			DM::Color(s)
		},
		Some(EDM::BVW {offset:o}) => {
			let s = ColorStatus {
				angle: o,
				mode: CSMode::BVW,
				mouse_position: (0.0,0.0)
			};
			DM::Color(s)
		},
		Some(EDM::BV {offset:o}) => {
			let s = ColorStatus {
				angle: o,
				mode: CSMode::BV,
				mouse_position: (0.0,0.0)
			};
			DM::Color(s)
		},
		Some(EDM::Smpte {repeat:r}) => {
			let s = ColorbarStatus {
				mode: CBSMode::SMPTE,
				repeat: r
			};
			DM::Colorbar(s)
		},
		Some(EDM::Arib {repeat:r}) => {
			let s = ColorbarStatus {
				mode: CBSMode::ARIB,
				repeat: r
			};
			DM::Colorbar(s)
		},
		Some(EDM::Mandelbrot) => DM::Mandelbrot,
		Some(EDM::Newton {p,tau,max}) => {
			DM::NewtonApprox(NewtonApproxStatus {p,tau,max})
		}
		Some(EDM::Help) => { panic!(); }
	};

	let aa = match &dm {
		DM::Ansi|DM::Colorbar(_) => 0,
		_ => a.aa
	};

	let mut s = Status {
		size: (0,0),
		draw_mode: dm,
		terminal: a.terminal,
		color: a.color,
		pixels: a.pixels,
		aa: aa,
		output: a.output.as_ref().map(|s| String::from(s))
	};

	if a.output!=None {
		match (a.width,a.height) {
			(None,None) => { error!("画像サイズが指定されていません"); },
			(None,Some(_)) => { error!("画像の横幅が指定されていません"); },
			(Some(_),None) => { error!("画像の縦幅が指定されていません"); },
			(Some(0),Some(0)) => { error!("画像サイズが不正です"); },
			(Some(0),Some(y)) if y>0 => { error!("画像の横幅が不正です"); },
			(Some(x),Some(0)) if x>0 => { error!("画像の縦幅が不正です"); },
			(Some(x),Some(y)) if x>0 && y>0 => { s.size = (x,y); },
			_ => { panic!(); }
		}
	}

	s
}