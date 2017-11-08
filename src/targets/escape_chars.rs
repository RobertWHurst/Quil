
pub fn escape_chars(i: &str, chars: &str) -> String {
  let mut o = String::new();
  let mut esc = false;
  for c in i.chars() {
    if !esc && chars.find(c).is_some() {
      o.push('\\');
    }
    esc = c == '\\';
    o.push(c);
  }
  o
}
