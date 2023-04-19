#[macro_export]
macro_rules! print
{
  ($($arg:tt)*) =>
  {
    // we can use up to 1023 letters
    let mut buf: [u8; 1024] = [0u8; 1024];
    match $crate::drivers::screen::write_to::show(&mut buf, format_args!($($arg)*))
    {
      Ok(text) => $crate::drivers::screen::_print(text),
      Err(error) => panic!("{}", error)
    }
  };
}

#[macro_export]
macro_rules! println
{
  () => ($crate::print!("\n"));
  ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! set_color
{
  () => ($crate::drivers::screen::set_color(7 as u8, 0 as u8));
  ($a:expr, $b:expr) => ($crate::drivers::screen::set_color($a as u8, $b as u8));
}