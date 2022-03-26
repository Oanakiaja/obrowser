mod css;
mod dom;
mod html;
fn main() {
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
    let document_tree = html::parse(source);
    println!("{:#?}", document_tree);

    let css_source = r#"
        h1, h2, h3 { margin: auto; color: #cc0000; }
        div.note { margin-bottom: 20px; padding: 10px; }
        #answer { display: none; }
    "#
    .to_string();
    let style_sheet = css::parse(css_source);
    println!("{:#?}", style_sheet);
}
