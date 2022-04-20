use console::Term;
use lib_rgb::*;
use ansi_term::Colour::RGB;


fn channelRender(channel: &Channel) {
    for pixel in channel.buffer.iter() {
        print!("{}", RGB( pixel.r, pixel.g, pixel.b).paint("#"));
    }

    println!();
}

fn main()  {    
    let term = Term::stdout();
    term.hide_cursor().unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    let mut engine = Engine::new([12,10,8,8,8,8,10,20], channelRender);
    engine.resize_channel(3, 13);

    while running.load(Ordering::SeqCst) {
        engine.update();
        engine.render();
        term.move_cursor_up(8).unwrap();
    }

    term.clear_screen().unwrap();
}


use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
