use lib_rgb::*;


fn channelRender(channel: &Box<Channel>) {
    println!("{}", (*(channel.as_ref())).buffer.len());
}

fn main() {
    // let c = lib_rgb::Colour {
    //     r: 255,
    //     g: 4,
    //     b: 255,
    // };
    // let chan = Box::new(Channel::new());
    // let channels = [
    //     Box::new(Channel::new()),
    //     Box::new(Channel::new()),
    //     Box::new(Channel::new()),
    //     Box::new(Channel::new()),
    //     Box::new(Channel::new()),
    //     Box::new(Channel::new()),
    //     Box::new(Channel::new()),
    //     Box::new(Channel::new()),
    //     ];
    let engine = Engine::new([12,10,8,8,8,8,10,20], channelRender);
    engine.render();
    // println!("{}", RGB(c.r, c.g, c.b).paint("Steel blue"));
}
