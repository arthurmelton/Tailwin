use std::process::Command;

pub fn on_startup() {
	Command::new("sh")
		.args(&["-c", "/usr/bin/start-all.sh"])
		.spawn()
		.expect("failed to execute process");
	Command::new("sh")
		.args(&["-c", "xmobar '/home/arthurmelton/.config/xmobar/xmobar.config'"])
		.spawn()
		.expect("failed to execute process");
	Command::new("alacritty")
		.spawn()
		.expect("failed to execute process");
}

pub fn on_key(key:u32, state:i8) -> String {
	match key {
		57 => {Command::new("firefox")
		.spawn()
		.expect("failed to execute process");},
		53 => {return "destroy".to_string();},
		_ => {Command::new("sh").args(&["-c", ["zenity --info --text='", key.to_string().as_str(), ":", state.to_string().as_str(), "'"].join("").as_str()]).spawn().expect("ERROR");},
	}
    return "null".to_string();
}

pub fn mod_mask() -> i8 {
	return 4;
}