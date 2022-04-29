#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(array_chunks)]

extern crate alloc;

mod renderer;
use renderer::PicoRenderer;


use fixed::types::I16F16;
use alloc::boxed::Box;
use alloc_cortex_m::CortexMHeap;
use hal::Timer;
use hal::gpio::{Pin, FunctionPio0};
use lib_rgb::graphics::gradient::UnicornVomit;
use lib_rgb::graphics::{ChaseShader};
use core::alloc::Layout;
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
use embedded_hal::timer::CountDown;

// // Ensure we halt the program on panic (if we don't mention this crate it won't
// // be linked)
// use panic_halt as _;

// Pull in any important traits

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;
use hal::pio::{PIOExt};

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

/// USB SHIT
// Pull in any important traits
use embedded_time::fixed_point::FixedPoint;
// use rp_pico::hal::prelude::*;
// The macro for marking our interrupt functions
use rp_pico::hal::pac::interrupt;
// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Human Interface Device (HID) Class support
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::MouseReport;
use usbd_hid::hid_class::HIDClass;

/// The USB Device Driver (shared with the interrupt).
static mut USB_DEVICE: Option<UsbDevice<hal::usb::UsbBus>> = None;

/// The USB Bus Driver (shared with the interrupt).
static mut USB_BUS: Option<UsbBusAllocator<hal::usb::UsbBus>> = None;

/// The USB Human Interface Device Driver (shared with the interrupt).
static mut USB_HID: Option<HIDClass<hal::usb::UsbBus>> = None;


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
    
    let _core: pac::CorePeripherals = pac::CorePeripherals::take().unwrap();

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


    // USB SHIT
    // the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_BUS = Some(usb_bus);
    }

    // Grab a reference to the USB Bus allocator. We are promising to the
    // compiler not to take mutable access to this global variable whilst this
    // reference exists!
    let bus_ref = unsafe { USB_BUS.as_ref().unwrap() };

    // Set up the USB HID Class Device driver, providing Mouse Reports
    let usb_hid = HIDClass::new(bus_ref, MouseReport::desc(), 100);
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet.
        USB_HID = Some(usb_hid);
    }

    // Create a USB device with a fake VID and PID
    let usb_dev = UsbDeviceBuilder::new(bus_ref, UsbVidPid(0x16c0, 0x27da))
        .manufacturer("Fake company")
        .product("Twitchy Mousey")
        .serial_number("TEST")
        .device_class(0xEF) // misc
        .build();
    unsafe {
        // Note (safety): This is safe as interrupts haven't been started yet
        USB_DEVICE = Some(usb_dev);
    }

    unsafe {
        // Enable the USB interrupt
        pac::NVIC::unmask(hal::pac::Interrupt::USBCTRL_IRQ);
    };




    
    // Set the LED to be an output
    let mut led_pin = pins.gpio0.into_push_pull_output();
    
    let _led: Pin<_, FunctionPio0> = pins.gpio16.into_mode();
    // let mut pico = PicoRgb::new(led_pin, delay);

    let (mut pio0, sm0, _sm1, _sm2, _sm3) = pac.PIO0.split(&mut pac.RESETS);
    let mut engine = Engine::new([9, 16, 16, 16, 16, 16, 16, 1]);
    let unicorn_vomit = alloc::rc::Rc::new(UnicornVomit{});
    engine.set_shader(0, Box::new(ChaseShader::new(unicorn_vomit)));
    engine.set_renderer(0, Box::new(PicoRenderer::new(16, &mut pio0, sm0)));
    
    let frame_rate: Milliseconds::<u32> = Hertz::<u32>(10).to_duration().unwrap();
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

        
        let rep_up = MouseReport {
            x: 0,
            y: 0,
            buttons: 0,
            wheel: 0,
            pan: 0,
        };
        push_mouse_movement(rep_up).ok().unwrap_or(0);
    }
}

/// Submit a new mouse movement report to the USB stack.
///
/// We do this with interrupts disabled, to avoid a race hazard with the USB IRQ.
fn push_mouse_movement(report: MouseReport) -> Result<usize, usb_device::UsbError> {
    cortex_m::interrupt::free(|_| unsafe {
        // Now interrupts are disabled, grab the global variable and, if
        // available, send it a HID report
        USB_HID.as_mut().map(|hid| hid.push_input(&report))
    })
    .unwrap()
}


/// This function is called whenever the USB Hardware generates an Interrupt
/// Request.
#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // Handle USB request
    let usb_dev = USB_DEVICE.as_mut().unwrap();
    let usb_hid = USB_HID.as_mut().unwrap();
    usb_dev.poll(&mut [usb_hid]);
}
    
#[alloc_error_handler]
fn oom(_: Layout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // println!("{}", info);
    loop {}
}

// End of file