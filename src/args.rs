use clap::{Parser,Subcommand,ArgEnum};
use crate::lib::*;

#[derive(Parser)]
/// 様々な絵をターミナルに表示させます。
/// サブコマンドにより描画内容を指定します。指定しない場合は ansi が選択されます。
/// それぞれのサブコマンドのオプションは visualizer help <SUBCOMMAND> で確認します。
/// ターミナルがフルカラーに対応していることを検知すれば自動的にフルカラーで描画します。
pub struct Args {
	#[clap(subcommand)]
	/// 描画モードに関するキーワード
	pub draw_mode: Option<DrawMode>,
	#[clap(short,long)]
	/// 画像ファイルで指定する場合、出力先のパスを指定します。指定しない場合はターミナルに出力します。
	pub output: Option<String>,
	/// 画像ファイルの横幅を指定します。
	#[clap(long)]
	pub width: Option<u16>,
	/// 画像ファイルの縦幅を指定します。
	#[clap(long)]
	pub height: Option<u16>,
	/// ターミナルに出力する場合の出力形式 (テキスト/画像) を指定します。
	#[clap(short,long,arg_enum,default_value_t=TerminalMode::Texts)]
	pub terminal: TerminalMode,
	/// ターミナルへのテキスト出力に使用する色数を選択します。
	#[clap(long,arg_enum,default_value_t=TerminalColor::Auto)]
	pub color: TerminalColor,
	/// ターミナルへのテキスト出力の解像度を選択します。
	#[clap(long,arg_enum,default_value_t=TerminalPixels::Single)]
	pub pixels: TerminalPixels,
	/// アンチエイリアスのレベルを指定します。2以上の整数を指定するとアンチエイリアスが有効になります。ANSIテストとカラーバー以外で有効なオプションです。
	#[clap(long,default_value_t=2)]
	pub aa: u8,
	#[clap(long)]
	/// visualizer コマンドの使い方を表示します。
	pub help: bool
}

#[derive(Subcommand)]
pub enum DrawMode {
	/// ターミナルのANSIカラー表示をテストします。
	Ansi,
	/// 中心が白の色相環を描画します。
	Hue {
		#[clap(short,long,default_value_t = 0.0)]
		/// 色相環を回転させる角度を度数法により指定します。
		rotate: f64
	},
	/// 色相環を描画します。
	Conic {
		#[clap(short,long,default_value_t = 0.0)]
		/// 色相環を回転させる角度を度数法により指定します。
		rotate: f64
	},
	/// 横方向を色相、縦方向をHSL明度として描画します。
	BVW {
		#[clap(short,long,default_value_t = 0.0)]
		/// 図を横方向でずらす際の色相角度を度数法により指定します。
		offset: f64
	},
	/// 横方向を色相、縦方向をHSV明度として描画します。
	BV {
		#[clap(short,long,default_value_t = 0.0)]
		/// 図を横方向でずらす際の色相角度を度数法により指定します。
		offset: f64
	},
	/// アナログ放送用のSMPTEカラーバーを描画します。
	Smpte {
		#[clap(short,long)]
		/// 図を繰り返します。
		repeat: bool
	},
	/// マンデルブロ集合を描画します。
	Mandelbrot,
	/// 1の p 乗根のニュートン近似の収束先を描画します。
	Newton {
		#[clap(short,default_value_t = 6)]
		/// 次数を指定します。
		p: usize,
		#[clap(short,long,default_value_t = 0.1)]
		/// 収束因子 τ を指定します。
		tau: f64,
		#[clap(short,long,default_value_t = 1000)]
		/// 最大イテレート回数を指定します。この回数を超えても収束しない場合は黒色になります。
		max: usize
	},
	/// デジタル放送用ARIBカラーバーを描画します。
	Arib {
		#[clap(short,long)]
		/// 図を繰り返します。
		repeat: bool
	},
	/// visualizer コマンドの使い方を表示します。
	Help
}

#[derive(ArgEnum,Clone)]
pub enum TerminalMode {
	Texts,
	Image
}

#[derive(ArgEnum,Clone)]
pub enum TerminalColor {
	Auto,
	Less,
	Full,
}

#[derive(ArgEnum,Clone)]
pub enum TerminalPixels {
	Single,
	Double,
}

pub fn init_status(a:&Args) -> Status {

	let dm = match a.draw_mode {
		Some(DrawMode::Ansi)|None => DM::Ansi,
		Some(DrawMode::Hue {rotate:r}) => {
			let s = ColorStatus {
				angle: r,
				mode:  CSMode::ConicW,
				mouse_position: (0.0,0.0)
			};
			DM::Color(s)
		},
		Some(DrawMode::Conic {rotate:r}) => {
			let s = ColorStatus {
				angle: r,
				mode: CSMode::Conic,
				mouse_position: (0.0,0.0)
			};
			DM::Color(s)
		},
		Some(DrawMode::BVW {offset:o}) => {
			let s = ColorStatus {
				angle: o,
				mode: CSMode::BVW,
				mouse_position: (0.0,0.0)
			};
			DM::Color(s)
		},
		Some(DrawMode::BV {offset:o}) => {
			let s = ColorStatus {
				angle: o,
				mode: CSMode::BV,
				mouse_position: (0.0,0.0)
			};
			DM::Color(s)
		},
		Some(DrawMode::Smpte {repeat:r}) => {
			let s = ColorbarStatus {
				mode: CBSMode::SMPTE,
				repeat: r
			};
			DM::Colorbar(s)
		},
		Some(DrawMode::Arib {repeat:r}) => {
			let s = ColorbarStatus {
				mode: CBSMode::ARIB,
				repeat: r
			};
			DM::Colorbar(s)
		},
		Some(DrawMode::Mandelbrot) => DM::Mandelbrot,
		Some(DrawMode::Newton {p,tau,max}) => {
			DM::NewtonApprox(NewtonApproxStatus {p,tau,max})
		}
		Some(DrawMode::Help) => { panic!(); }
	};

	let aa = match &dm {
		DM::Ansi|DM::Colorbar(_) => 0,
		_ => a.aa
	};

	let mut s = Status {
		size: (0,0),
		draw_mode: dm,
		terminal: match a.terminal {
			TerminalMode::Texts => TM::Texts,
			TerminalMode::Image => TM::Image
		},
		color: match a.color {
			TerminalColor::Auto => TC::Auto,
			TerminalColor::Less => TC::Less,
			TerminalColor::Full => TC::Full
		},
		pixels: match a.pixels {
			TerminalPixels::Single => TP::Single,
			TerminalPixels::Double => TP::Double
		},
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