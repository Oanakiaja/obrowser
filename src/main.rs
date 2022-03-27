mod canvas;
mod css;
mod display;
mod dom;
mod html;
mod layout;
mod style;
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
    // println!("{:#?}", canvas);
}
