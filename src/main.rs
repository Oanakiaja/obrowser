mod css;
mod dom;
mod html;
mod layout;
mod style;
fn main() {
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let source = r#"
    <div class="a" id="app">
        <div class="b">
          <div class="c">
            123
            <div class="d"> </div>
          </div>
        </div>
    </div>"#
        .to_string();

    let css_source = r#"
        .a, .b, .c { margin: auto; color: #cc0000; }
        div.a { margin-bottom: 20px; padding: 10px; }
        #app { padding: 10px; }
    "#
    .to_string();

    let document_tree = html::parse(source);
    // println!("{:#?}", document_tree);

    let style_sheet = css::parse(css_source);
    // println!("{:#?}", style_sheet);

    let style_tree = style::style_tree(&document_tree, &style_sheet);
    // println!("{:#?}", style_tree);

    let layout_tree = layout::layout_tree(&style_tree, viewport);
    println!("{:#?}", layout_tree);
}
