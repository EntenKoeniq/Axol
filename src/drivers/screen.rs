static mut FRAMEBUFFER: *mut u8 = 0xb8000 as *mut u8;
static mut SCREEN_COL: u16 = 0;
static mut SCREEN_ROW: u16 = 0;
static mut SCREEN_SCHEME: u8 = 0;

const VGA_WIDTH: u16 = 80;
const VGA_HEIGHT: u16 = 25;

pub(crate) fn set_color(fg: u8, bg: u8)
{
  unsafe
  {
    SCREEN_SCHEME = fg | bg << 4;
  }
}

pub(crate) fn screen_clear()
{
  for y in 0..VGA_HEIGHT
  {
    for x in 0..VGA_WIDTH
    {
      print_char_at(' ', x, y);
    }
  }
}

fn print_char_at(c: char, x: u16, y: u16)
{
  let offset: isize = 2 * (y * VGA_WIDTH + x) as isize;
  
  unsafe
  {
    *FRAMEBUFFER.offset(offset) = c as u8;
    *FRAMEBUFFER.offset(offset + 1) = SCREEN_SCHEME;
  }
}

fn print_char(c: char)
{
  match c
  {
    '\n' =>
    {
      unsafe
      {
        SCREEN_COL = 0;
        SCREEN_ROW = SCREEN_ROW + 1;
      }
    },
    _ =>
    {
      unsafe
      {
        print_char_at(c, SCREEN_COL, SCREEN_ROW);
        
        SCREEN_COL = SCREEN_COL + 1;
        if SCREEN_COL != VGA_WIDTH
        {
          return;
        }
        
        SCREEN_COL = 0;
        
        SCREEN_ROW = SCREEN_ROW + 1;
        if SCREEN_ROW == VGA_HEIGHT
        {
          SCREEN_ROW = 0;
        }
      }
    }
  }
}

#[doc(hidden)]
pub(crate) fn _print(text: &str)
{
  for c in text.chars()
  {
    print_char(c);
  }
}

#[doc(hidden)]
pub(crate) mod write_to
{
  use core::fmt;
  
  pub(crate) struct WriteTo<'a>
  {
    buffer: &'a mut [u8],
    used: usize
  }
  
  impl<'a> WriteTo<'a>
  {
    pub(crate) fn new(buffer: &'a mut [u8]) -> Self
    {
        WriteTo { buffer, used: 0 }
    }
    
    pub(crate) fn as_str(self) -> Option<&'a str>
    {
      if self.used > self.buffer.len()
      {
        return None;
      }
      
      // only successful concats of str - must be a valid str
      Some(unsafe { core::str::from_utf8_unchecked(&self.buffer[..self.used]) })
    }
  }
  
  impl<'a> fmt::Write for WriteTo<'a>
  {
    fn write_str(&mut self, s: &str) -> fmt::Result
    {
      if self.used > self.buffer.len()
      {
        return Err(fmt::Error);
      }
      
      let remaining_buf: &mut [u8] = &mut self.buffer[self.used..];
      let raw_s: &[u8] = s.as_bytes();
      let write_num: usize = core::cmp::min(raw_s.len(), remaining_buf.len());
      remaining_buf[..write_num].copy_from_slice(&raw_s[..write_num]);
      self.used += raw_s.len();
      
      if write_num < raw_s.len()
      {
        return Err(fmt::Error);
      }
      
      Ok(())
    }
  }
  
  pub(crate) fn show<'a>(buffer: &'a mut [u8], args: fmt::Arguments) -> Result<&'a str, fmt::Error>
  {
    let mut w: WriteTo = WriteTo::new(buffer);
    fmt::write(&mut w, args)?;
    w.as_str().ok_or(fmt::Error)
  }
}