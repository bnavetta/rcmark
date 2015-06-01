extern crate rcmark;

#[test]
fn test_render_html() {
    let doc = rcmark::parse_document("# Hi\n*This* is _CommonMark_",
                                     rcmark::DEFAULT);

    let rendered = rcmark::render_html(&doc, rcmark::DEFAULT);
    assert_eq!(rendered,
               "<h1>Hi</h1>\n<p><em>This</em> is <em>CommonMark</em></p>\n");
}

#[test]
fn test_render_man() {
    let doc = rcmark::parse_document("# Section\n**Text**", rcmark::DEFAULT);
    let rendered = rcmark::render_man(&doc, rcmark::DEFAULT);
    assert_eq!(rendered, ".SH\nSection\n.PP\n\\f[B]Text\\f[]\n");
}
