use lib_rgb::{graphics::Colour, Renderer};
use pio::SideSet;
use rp2040_hal::pio::{
    PIOExt, PinDir, ShiftDirection, StateMachineIndex, Tx, UninitStateMachine, PIO,
};

pub struct PicoRenderer<TPIO, TStateMachine>
where
    TPIO: PIOExt,
    TStateMachine: StateMachineIndex,
{
    // program: Program<32>,
    // state_machine: StateMachine<(TPIO, TStateMachine), Running>,
    out_stream: Tx<(TPIO, TStateMachine)>,
}

impl<TPIO, TStateMachine> PicoRenderer<TPIO, TStateMachine>
where
    TPIO: PIOExt,
    TStateMachine: StateMachineIndex,
{
    pub fn new(
        pin_id: u8,
        pio: &mut PIO<TPIO>,
        state_machine: UninitStateMachine<(TPIO, TStateMachine)>,
    ) -> Self {
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
        assembler.out_with_delay_and_side_set(pio::OutDestination::X, 1, T3 - 1, 0);
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
        let (mut _sm, _, tx) = rp2040_hal::pio::PIOBuilder::from_program(installed)
            .buffers(rp2040_hal::pio::Buffers::OnlyTx)
            .set_pins(pin_id, 1)
            .clock_divisor(div)
            .autopull(true)
            .pull_threshold(24)
            .side_set_pin_base(16)
            .out_shift_direction(ShiftDirection::Left)
            .build(state_machine);

        _sm.set_pindirs([(16, PinDir::Output)]);
        let _sm = _sm.start();

        PicoRenderer {
            // program,
            // state_machine: sm,
            out_stream: tx,
        }
    }
}

union Pixel {
    colour: Colour,
    data: u32,
}

impl<TPIO, TStateMachine> Renderer for PicoRenderer<TPIO, TStateMachine>
where
    TPIO: PIOExt,
    TStateMachine: StateMachineIndex,
{
    fn render(&mut self, channel: &[Colour]) {
        for c in channel {
            unsafe { while !self.out_stream.write(Pixel { colour: *c }.data << 8) {} }
        }
    }
}
