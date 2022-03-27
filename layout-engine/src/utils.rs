// #ffffff -> (255,255,255)
pub fn parse_color(color: &String) -> [u8; 3] {
  let mut pos = 1;
  let item = &color[pos..pos + 2];
  let r = u8::from_str_radix(item, 16).unwrap();
  pos += 2;
  let item = &color[pos..pos + 2];
  let g = u8::from_str_radix(item, 16).unwrap();
  pos += 2;
  let item = &color[pos..pos + 2];
  let b = u8::from_str_radix(item, 16).unwrap();
  [r, g, b]
}
