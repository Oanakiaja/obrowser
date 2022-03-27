mod canvas;
mod css;
mod display;
mod dom;
mod html;
mod layout;
mod style;

use image;

fn main() {
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let initial_containing_block = layout::Dimensions {
        content: layout::Rect {
            x: 0.0,
            y: 0.0,
            width: 800.0,
            height: 600.0,
        },
        padding: Default::default(),
        border: Default::default(),
        margin: Default::default(),
    };

    let source = r#"
    <div class="a">
      <div class="b">
        <div class="c">
          <div class="d">
            <div class="e">
              <div class="f">
                <div class="g">
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>"#
        .to_string();

    let css_source = r#"
      * { display: block; padding: 12px; }
      .a { background: #ff0000; }
      .b { background: #ffa500; }
      .c { background: #ffff00; }
      .d { background: #008000; }
      .e { background: #0000ff; }
      .f { background: #4b0082; }
      .g { background: #800080; }
    "#
    .to_string();

    let document_tree = html::parse(source);
    // println!("{:#?}", document_tree);

    let style_sheet = css::parse(css_source);
    // println!("{:#?}", style_sheet);

    let style_tree = style::style_tree(&document_tree, &style_sheet);
    // println!("{:#?}", style_tree);

    let layout_tree = layout::layout_tree(&style_tree, viewport);
    // println!("{:#?}", layout_tree);

    let display_list = display::build_display_list(&layout_tree);
    // println!("{:#?}", display_list);

    let canvas = canvas::paint(&display_list, initial_containing_block.content);

    let mut imgbuf = image::ImageBuffer::new(canvas.width as u32, canvas.height as u32);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb(parse_color(
            &canvas.pixels[(x as usize + canvas.width * y as usize)],
        ));
    }
    imgbuf.save("result.png").unwrap();
}

// #ffffff -> (255,255,255)
fn parse_color(color: &String) -> [u8; 3] {
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
