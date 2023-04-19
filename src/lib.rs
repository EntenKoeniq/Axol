#![no_main]
#![no_std]
#![feature(panic_info_message)]

use core::arch::asm;

#[macro_use]
mod macros;

mod unwind;
mod drivers;

#[no_mangle]
extern fn _start() -> !
{
  drivers::screen::screen_clear();
  
  set_color!(9, 0);
  println!(".---..-..-..----..-.");
  println!("| | | >  < | || || |__");
  println!("`-^-''-'`-``----'`----'\n");
  set_color!();
  
  println!("Loading...");
  
  // should call the panic_handler since we are using more than 1023 characters
  println!("{}", "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Duis autem vel eum iriure dolor in hendrerit in vulputate velit esse molestie consequat, vel illum dolore eu feugiat nulla facilisis a b");
  
  loop
  {
    hlt();
  }
}

// This allows the CPU to go into a sleep state where it consumes much less energy.
// Example: With an Intel Core i5 11400 in QEMU i get ~15% CPU usage without and ~0.2% with
// https://en.wikipedia.org/wiki/HLT_(x86_instruction)
#[doc(hidden)]
#[inline(always)]
pub(crate) fn hlt()
{
  unsafe { asm!("hlt"); }
}