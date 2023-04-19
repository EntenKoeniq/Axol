#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &::core::panic::PanicInfo<'_>) -> !
{
  // get the file and the line of the error
  let (file, line) = match info.location() {
    Some(loc) => (loc.file(), loc.line()),
    None => ("", 0)
  };

  set_color!(12, 0);
  if let Some(m) = info.message() {
    println!("PANIC file='{}', line='{}' :: {}", file, line, m);
  } else if let Some(m) = info.payload().downcast_ref::<&str>() {
    println!("PANIC file='{}', line='{}' :: {}", file, line, m);
  } else {
    println!("PANIC file='{}', line='{}' :: ?", file, line);
  }
  set_color!();

  loop
  {
    crate::hlt();
  }
}