use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::RcDom;

use crate::dom::{
    node::{Node, NodeData},
    tree::Tree,
};

static TEST_DOCUMENT: &'static str = "<!doctype html>
<html>
<head>
    <title>Test Document</title>
    <style>* {padding: 0}</style>
    <script>console.log('Hello, world')</script>
</head>
<body>
    <h1>Test Heading</h1>
    <p>Test Paragraph</p>
</body>
</html>";

fn parse_tree() -> Tree {
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut TEST_DOCUMENT.as_bytes())
        .unwrap()
        .document
        .into()
}

fn assert_element_name(node: &Node, expected_name: &str) {
    match &node.data {
        NodeData::Element { name, .. } => assert_eq!(name, expected_name),
        data @ _ => panic!("expected an element, got {:?}", data),
    }
}

fn assert_text_contents(node: &Node, expected_text: &str) {
    match &node.data {
        NodeData::Text(contents) => assert_eq!(contents, expected_text),
        data @ _ => panic!("expected a text node, got {:?}", data),
    }
}

#[test]
fn test_conversion() {
    let tree = parse_tree();
    dbg!(&tree);

    let document = tree.get_root_node();

    assert!(
        matches!(document.data, NodeData::Document),
        "Top-level node should be a document node"
    );

    let doctype = document.get_child(&tree, 0);
    assert!(
        matches!(doctype.data, NodeData::Doctype { .. }),
        "First child of the document should be a doctype node"
    );

    let html = document.get_child(&tree, 1);
    assert_element_name(html, "html");

    let head = html.get_child(&tree, 0);
    assert_element_name(head, "head");

    let title = head.get_child(&tree, 1);
    assert_element_name(title, "title");
    assert_text_contents(title.get_child(&tree, 0), "Test Document");

    let style = head.get_child(&tree, 3);
    assert_element_name(style, "style");
    assert_text_contents(style.get_child(&tree, 0), "* {padding: 0}");

    let script = head.get_child(&tree, 5);
    assert_element_name(script, "script");
    assert_text_contents(script.get_child(&tree, 0), "console.log('Hello, world')");

    let body = html.get_child(&tree, 2);
    assert_element_name(body, "body");

    let h1 = body.get_child(&tree, 1);
    assert_element_name(h1, "h1");
    assert_text_contents(h1.get_child(&tree, 0), "Test Heading");

    let p = body.get_child(&tree, 3);
    assert_element_name(p, "p");
    assert_text_contents(p.get_child(&tree, 0), "Test Paragraph");
}
