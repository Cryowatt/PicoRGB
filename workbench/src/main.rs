extern crate alloc;

use console::Term;

use ansi_term::Colour::RGB;
use lib_rgb::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn channel_render(channel: &Channel) {
    for pixel in channel.buffer.iter() {
        print!("{}", RGB(pixel.r, pixel.g, pixel.b).paint("■"));
    }

    println!();
}

fn main() {
    let term = Term::stdout();
    term.hide_cursor().unwrap();
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let unicorn_vomit = alloc::rc::Rc::new(UnicornVomit{});

    let mut engine = Engine::new([2, 3, 5, 10, 20, 40, 80, 160], channel_render);
    engine.set_shader(0, Box::new(ChaseShader::new(unicorn_vomit.clone())));
    engine.set_shader(1, Box::new(ChaseShader::new(unicorn_vomit.clone())));
    engine.set_shader(2, Box::new(ChaseShader::new(unicorn_vomit.clone())));
    engine.set_shader(3, Box::new(ChaseShader::new(unicorn_vomit.clone())));
    engine.set_shader(4, Box::new(ChaseShader::new(unicorn_vomit.clone())));
    engine.set_shader(5, Box::new(ChaseShader::new(unicorn_vomit.clone())));
    engine.set_shader(6, Box::new(ChaseShader::new(unicorn_vomit.clone())));
    engine.set_shader(7, Box::new(ChaseShader::new(unicorn_vomit)));
    engine.update(0);
    engine.render();
    println!();
    const LOOP_TARGET: Duration = Duration::from_millis(10);

    while running.load(Ordering::SeqCst) {
        let now = Instant::now();
        term.move_cursor_up(9).unwrap();
        engine.update(LOOP_TARGET.as_millis().try_into().unwrap());
        engine.render();

        if now.elapsed() < LOOP_TARGET {
            let loop_padding = LOOP_TARGET - now.elapsed();
            println!("{}μs", loop_padding.as_micros());
            thread::sleep(loop_padding);
        } else {
            println!("SKIP");
        }
    }
    term.show_cursor().unwrap();
}
