#![no_std]
// #![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt;
extern crate stm32f41x;
extern crate panic_abort;

use cortex_m_rt::ExceptionFrame;

#[inline(never)]
fn delay(tim6: &stm32f41x::TIM6, delay_time: u16) {
    tim6.arr.write(|w| unsafe { w.arr().bits(delay_time) } );
    tim6.cr1.modify(|_, w| w.cen().set_bit());

    while !tim6.sr.read().uif().bit_is_set() {}

    tim6.sr.modify(|_, w| w.uif().clear_bit())
}

entry!(main);

fn main() -> ! {
    let stm_peripherals = stm32f41x::Peripherals::take().unwrap();

    let gpioa = stm_peripherals.GPIOA;
    let rcc = stm_peripherals.RCC;
    let tim6 = stm_peripherals.TIM6;

    rcc.ahb1enr.modify(|_, w| w.gpioaen().set_bit());
    rcc.apb1enr.modify(|_, w| w.tim6en().set_bit());

    gpioa.moder.modify(|_, w| unsafe { w.moder8().bits(0b01) } );
    tim6.cr1.write(|w| w.opm().clear_bit().cen().set_bit());
    tim6.psc.write(|w| unsafe { w.psc().bits(7_999) } );

    loop {
        gpioa.odr.write(|w| w.odr8().set_bit());
        delay(&tim6, 500_u16);
        gpioa.odr.write(|w| w.odr8().clear_bit());
        delay(&tim6, 500_u16);
    }

}

// define the hard fault handler
exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

// define the default exception handler
exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}
