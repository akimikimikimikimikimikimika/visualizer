/// プリミティブ型のタプル
mod primitive {

	/// 非負整数による座標型
	pub type CU = (u16,u16);
	/// 実数による座標型
	pub type CF = (f64,f64);

	/// 色を表現する型
	pub enum C {
		Float{r:f64,g:f64,b:f64,a:f64},
		Int{r:u8,g:u8,b:u8,a:u8},
		GFloat{v:f64,a:f64},
		Reverse,
		None,
		Ansi(u8),KL,KD,RL,RD,GL,GD,YL,YD,BL,BD,ML,MD,CL,CD,WL,WD
	}

}
pub use primitive::*;



// 共通のデータ型
mod common {

	use crate::data::primitive::*;

	/// ステータス管理構造体
	pub struct Status {
		pub size:      CU,
		pub draw_mode: DM,
		pub terminal:  TM,
		pub color:     TC,
		pub pixels:    TP,
		pub aa:        u8,
		pub output:    Option<String>
	}

	/// 描画モードとそれぞれのオプション
	mod draw_mode {

		use crate::data::primitive::*;

		/// 描画モードの指定
		pub enum DrawMode {
			Ansi,
			Color(ColorStatus),
			Colorbar(ColorbarStatus),
			Mandelbrot,
			NewtonApprox(NewtonApproxStatus)
		}
		pub type DM = DrawMode;

		pub struct ColorStatus {
			pub angle: f64,
			pub mode:  CSMode,
			pub mouse_position: CF
		}

		pub enum CSMode {
			ConicW,
			Conic,
			BVW,
			BV
		}

		pub struct ColorbarStatus {
			pub mode:   CBSMode,
			pub repeat: bool
		}
		pub enum CBSMode {
			SMPTE,
			ARIB
		}

		pub struct NewtonApproxStatus {
			pub p:     usize,
			pub tau:   f64,
			pub max:   usize,
			pub speed: bool
		}

	}
	pub use draw_mode::*;

	/// 描画モードに依らない共通のオプション
	mod common_options {

		use clap::ArgEnum;

		#[derive(ArgEnum,Clone,Copy)]
		pub enum TerminalMode {
			Texts,
			Image
		}
		pub type TM = TerminalMode;

		#[derive(ArgEnum,Clone,Copy)]
		pub enum TerminalColor {
			Auto,
			Less,
			Full,
		}
		pub type TC = TerminalColor;

		#[derive(ArgEnum,Clone,Copy)]
		pub enum TerminalPixels {
			Single,
			Double,
		}
		pub type TP = TerminalPixels;

	}
	pub use common_options::*;

}
pub use common::*;



// 外部向けデータ型
mod external {

	use clap::{Parser,Subcommand};
	use crate::data::common::*;

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
		/// 画像ファイルとして出力する場合の出力先のパスを指定します。指定しない場合はターミナルに出力します。
		pub output: Option<String>,
		/// 画像ファイルの横幅を指定します。ターミナルモードでは無視されます。
		#[clap(long)]
		pub width: Option<u16>,
		/// 画像ファイルの縦幅を指定します。ターミナルモードでは無視されます。
		#[clap(long)]
		pub height: Option<u16>,
		/// ターミナルに出力する場合の出力形式 (テキスト/画像) を指定します。画像は一部のターミナルでのみ対応しています。
		#[clap(short,long,arg_enum,default_value_t=TM::Texts)]
		pub terminal: TM,
		/// ターミナルへのテキスト出力に使用する色数を選択します。 auto はターミナルがフルカラーに対応していればフルカラーにします。 less, full はターミナルの対応如何に依らず強制的に設定を適用します。
		#[clap(long,arg_enum,default_value_t=TC::Auto)]
		pub color: TC,
		/// ターミナルへのテキスト出力の解像度を選択します。ターミナルによっては double の解像度で適切に表示されません。
		#[clap(long,arg_enum,default_value_t=TP::Single)]
		pub pixels: TP,
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
		/// 1の p 乗根のニュートン近似の収束先の偏角を色相として描画します。
		Newton {
			#[clap(short,default_value_t = 6)]
			/// p 乗根の次数を指定します。
			p: usize,
			#[clap(short,long,default_value_t = 0.1)]
			/// 収束因子 τ を指定します。
			tau: f64,
			#[clap(short,long,default_value_t = 1000)]
			/// 最大イテレート回数を指定します。この回数を超えても収束しない場合は黒色になります。
			max: usize,
			#[clap(short,long)]
			/// 収束までに要した時間により色分けします。収束までより時間を要した点ほど暗い値になります。
			speed: bool
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
	pub type EDM = DrawMode;

}
pub use external::*;