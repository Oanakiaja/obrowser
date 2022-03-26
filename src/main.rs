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
    println!("{:?}", document_tree);
}
