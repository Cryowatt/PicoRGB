use ansi_hex_color;

fn main() {
	let foreground = "#FF0000";
	let background = "#004082";
	let txt = "Hello world";
	
	let colored_txt = ansi_hex_color::colored(
		foreground, background, txt);
        
	println!("{}", colored_txt);
}