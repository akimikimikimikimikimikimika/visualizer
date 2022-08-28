#[macro_export]
macro_rules! error {
	($msg:expr) => {{
		eprintln!($msg);
		std::process::exit(1);
	}};
}

#[macro_export]
macro_rules! try_catch {
	($trial:expr) => {
		if let Err(e) = $trial {
			eprintln!("エラーが発生しました:\n{:?}",e);
			std::process::exit(1);
		}
	};
}