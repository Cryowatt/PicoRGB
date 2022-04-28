//! # Pico Blinky Example
//!
//! Blinks the LED on a Pico board.
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for
//! the on-board LED.
//!
//! See the `Cargo.toml` file for Copyright and licence details.

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(array_chunks)]

extern crate alloc;

use fixed::types::I16F16;
use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use cortex_m::delay::Delay;
use hal::Timer;
use hal::clocks::SystemClock;
// use embedded_hal::digital::v2::OutputPin;
use hal::gpio::{PushPull, Pin, Output, PushPullOutput, PinId, FunctionPio0};
use hal::gpio::bank0::Gpio25;
use pio::{Program, SideSet};
// use hal::pio;
use core::alloc::Layout;
use core::iter::Inspect;
use core::panic::PanicInfo;
use lib_rgb::*;

// The macro for our start-up function
use cortex_m_rt::entry;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();


// GPIO traits
use embedded_hal::digital::v2::OutputPin;

// Time handling traits
use embedded_time::duration::*;    // imports all duration-related types and traits
use embedded_time::rate::*;        // imports all rate-related types and traits
use embedded_time::clock;
use embedded_hal::timer::CountDown;
use embedded_time::duration::Extensions;

// // Ensure we halt the program on panic (if we don't mention this crate it won't
// // be linked)
// use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;
use hal::pio::{PIOExt, Tx, StateMachineIndex, PIO, UninitStateMachine, StateMachine, Running, ShiftDirection, PinDir};

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

struct PicoRgb<TPinId> where TPinId: PinId {
    led_pin: Pin<TPinId, Output<PushPull>>,
    delay: Delay,
}

struct PicoRenderer<TPIO, TStateMachine>
    where TPIO: PIOExt, TStateMachine: StateMachineIndex {
    // program: Program<32>,
    state_machine: StateMachine<(TPIO, TStateMachine), Running>,
    out_stream: Tx<(TPIO, TStateMachine)>,
}

impl<TPIO, TStateMachine> PicoRenderer<TPIO, TStateMachine>
    where TPIO: PIOExt, TStateMachine: StateMachineIndex {
    fn new(pin_id: u8, pio: &mut PIO<TPIO>, state_machine: UninitStateMachine<(TPIO, TStateMachine)> ) -> Self
    {
        const T1: u8 = 2;
        const T2: u8 = 5;
        const T3: u8 = 3;
        
        let mut assembler = pio::Assembler::<32>::new_with_side_set(SideSet::new(false, 1, false));
        let mut wrap_target = assembler.label();
        let mut wrap_source = assembler.label();
        let mut bit_loop = assembler.label();
        let mut do_zero = assembler.label();

        assembler.bind(&mut wrap_target);
        assembler.bind(&mut bit_loop);
        assembler.out_with_delay_and_side_set(pio::OutDestination::X, 1, T3 -  1, 0);
        assembler.jmp_with_delay_and_side_set(pio::JmpCondition::XIsZero, &mut do_zero, T1 - 1, 1);
        assembler.jmp_with_delay_and_side_set(pio::JmpCondition::Always, &mut bit_loop, T2 - 1, 1);
        assembler.bind(&mut do_zero);
        assembler.nop_with_delay_and_side_set(T2 - 1, 0);

        assembler.bind(&mut wrap_source);
        let program = assembler.assemble_with_wrap(wrap_source, wrap_target);
                
        // TODO: Add uninstaller/deconstructor. Or maybe not, why destroy the renderer?

        // Initialize and start PIO
        let installed = pio.install(&program).unwrap();
        let div = 16f32; //8f32 / 133f32; // as slow as possible (0 is interpreted as 65536)
        let (mut sm, _, tx) = rp2040_hal::pio::PIOBuilder::from_program(installed)
            .set_pins(16, 1)
            .clock_divisor(div)
            .autopull(true)
            .pull_threshold(24)
            .side_set_pin_base(16)
            .out_shift_direction(ShiftDirection::Left)
            .build(state_machine);

        sm.set_pindirs([(16, PinDir::Output)]);
        let sm = sm.start();

        PicoRenderer {
            // program,
            state_machine: sm,
            out_stream: tx,
        }
    }
}

union Pixel {
    colour: Colour,
    data: u32,
}

impl <TPIO, TStateMachine> Renderer for PicoRenderer<TPIO, TStateMachine>
    where TPIO: PIOExt, TStateMachine: StateMachineIndex {
    fn render(&mut self, channel: &[Colour]) { 
        for c in channel {
            unsafe {
                while !self.out_stream.write(Pixel { colour: *c }.data << 8) {}
            }
        }
        // let c = [Colour{g:1, r:2, b:3},Colour{g:4, r:5, b:6},Colour{g:7, r:8, b:9},Colour{g:10, r:11, b:12},Colour{g:13, r:14, b:15}];

        // // let p = c.as_ptr();
        // unsafe {
        //     while !self.out_stream.write(Pixel { colour: c[0] }.data << 8) {}
        //     while !self.out_stream.write(Pixel { colour: c[1] }.data << 8) {}
        //     while !self.out_stream.write(Pixel { colour: c[2] }.data << 8) {}
        //     // while !self.out_stream.write(*(p.offset(1) as *const u32)) {}
        //     // while !self.out_stream.write(*(p.offset(2) as *const u32)) {}
        // }
        // for c in foo.chunks_exact(4) {
            // let w0 = u32::from_ne_bytes([c[0].g, c[0].r, c[0].b, c[1].g]);
            // let w1 = u32::from_ne_bytes([c[1].r, c[1].b, c[2].g, c[2].r]);
            // let w2 = u32::from_ne_bytes([c[2].b, c[3].g, c[3].r, c[3].b]);
            // // assert!(w0 == 0x01020304);
            // // assert!(w1 == 0x05060708);
            // // assert!(w2 == 0x090a0b0c);
            // while !self.out_stream.write(w0) {}
            // while !self.out_stream.write(w1) {}
            // while !self.out_stream.write(w2) {}
        // }
        
        // unsafe{
        //     let cd = foo.as_ptr() as *const u32;
        //     // let cd = core::slice::from_raw_parts(foo.as_ptr() as *const u32, foo.len());
        //     for i in 0..(foo.len() * 3 / 4) as isize {            
        //         self.out_stream.write((*cd.offset(i)).swap_bytes());
        //     }
        //     // let cd = core::slice::from_raw_parts(foo.as_ptr() as *const u32, foo.len());
        //     // for p in cd {
        //     //     self.out_stream.write((*p).swap_bytes());
        //     // }
        // }
        // Chunk into 4 pixels to align 4*24bit into 3*32bit
        // let iter = foo.array_chunks::<4>();
        // for chunk in iter {
        //     unsafe {
        //         let data = to_pio_fifo(chunk);

        //         for d in data {
        //             self.out_stream.write((*d).swap_bytes());
        //             //while !self.out_stream.write((*d).swap_bytes()) {}
        //         }
        //     }
        // }

        // self.out_stream.write(0x00FF_00FFu32);
        // self.out_stream.write(0x0000_0000u32);
        // self.out_stream.write(0xFF00_FFFFu32);
        // for pixel_chunk in channel.chunks(4) {
        //     unsafe {
        //         let shit = pixel_chunk.align_to::<u32>();
        //         while !self.out_stream.write(pixel_chunk.align_to::<u32>()) {}            
        //     }
        // }
        // for pixel in channel.iter() {
            //while !self.out_stream.write(pixel) {}
            // while !self.out_stream.write(Colour::RED){}
            // while !self.out_stream.write(Colour::YELLOW){}
            // while !self.out_stream.write(Colour::GREEN){}
            // while !self.out_stream.write(Colour::CYAN){}
            // while !self.out_stream.write(Colour::BLUE){}
            // while !self.out_stream.write(Colour::MAGENTA){}
        // }
    }
}


// struct PicoRgb;
impl<TPinId> PicoRgb<TPinId> where TPinId: PinId {
    pub fn new(led_pin: Pin<TPinId, Output<PushPull>>, delay: Delay) -> Self
    {        
        PicoRgb::<TPinId> {
            led_pin,
            delay,
        }
    }

    // pub fn run(&mut self) -> ! {
    
    //     // The delay object lets us wait for specified amounts of time (in
    //     // milliseconds)
    //     // let mut delay = cortex_m::delay::Delay;
        
    //     // let channel_render = |channel: &Channel| {
    //     //     led_pin.set_high().unwrap();
    //     //     delay.delay_ms(100);
    //     //     led_pin.set_low().unwrap();
    //     //     delay.delay_ms(100);
    //     // };
    
    //     // let fp: fn(&Channel) = channel_render.call;
    // }
}

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then blinks the LED in an
/// infinite loop.
#[entry]
fn main() -> ! {
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe {
            ALLOCATOR.init((&mut HEAP).as_ptr() as usize, HEAP_SIZE)
        }
    }

    // Grab our singleton objects
    let mut pac: pac::Peripherals = pac::Peripherals::take().unwrap();
    let core: pac::CorePeripherals = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);
    
    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    
    let timer = Timer::new(pac.TIMER, &mut pac.RESETS);
    //let framerate = embedded_time::rate::Hertz::
    // let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    
    // Set the LED to be an output
    let mut led_pin = pins.gpio0.into_push_pull_output();
    
    let _led: Pin<_, FunctionPio0> = pins.gpio16.into_mode();
    // let mut pico = PicoRgb::new(led_pin, delay);

    let (mut pio0, sm0, sm1, sm2, sm3) = pac.PIO0.split(&mut pac.RESETS);

    let mut engine = Engine::new([9, 16, 16, 16, 16, 16, 16, 1]);
    let unicorn_vomit = alloc::rc::Rc::new(UnicornVomit{});
    engine.set_shader(0, Box::new(ChaseShader::new(unicorn_vomit)));
    engine.set_renderer(0, Box::new(PicoRenderer::new(16, &mut pio0, sm0)));
    
    let frame_rate: Milliseconds::<u32> = Hertz::<u32>(100).to_duration().unwrap();
    // let LOOP_TARGET = Milliseconds::<u32>(20);
    // let timer = Instant::<Clock>::new(0);
    let mut delay = timer.count_down();

    loop {
        delay.start(frame_rate);
        led_pin.set_high().unwrap();
        engine.update(I16F16::from_num(frame_rate.integer()) / 1000);
        engine.render();
        led_pin.set_low().unwrap();        
        let _ = nb::block!(delay.wait());
        
        // led_pin.set_high().unwrap();
        // engine.update(33);
        // engine.render();
        // led_pin.set_low().unwrap();
        // // let len = xs.len() as u32;
        // delay.delay_ms(100);
        // delay.delay_ms(900);
        // // delay.delay_ms(100 * len);
    }
}
    
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}

// End of file