use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::RcDom;

use crate::dom::node::{Node, NodeData};

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

fn parse_tree() -> Node {
    parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut TEST_DOCUMENT.as_bytes())
        .unwrap()
        .document
        .into()
}

fn assert_element_name(node: Node, expected_name: &str) {
    match node.get_node_data().as_ref() {
        NodeData::Element { name, .. } => assert_eq!(name, expected_name),
        data @ _ => panic!("expected an element, got {:?}", data),
    }
}

fn assert_text_contents(node: Node, expected_text: &str) {
    match node.get_node_data().as_ref() {
        NodeData::Text(contents) => assert_eq!(contents, expected_text),
        data @ _ => panic!("expected a text node, got {:?}", data),
    }
}

#[test]
fn test_conversion() {
    let tree = parse_tree();
    dbg!(&tree);

    assert!(
        matches!(tree.get_node_data().as_ref(), NodeData::Document),
        "Top-level node should be a document node"
    );

    let doctype = tree.get_child(0);
    assert!(
        matches!(doctype.get_node_data().as_ref(), NodeData::Doctype { .. }),
        "First child of the document should be a doctype node"
    );

    let html = tree.get_child(1);
    assert_element_name(html.clone(), "html");

    let head = html.get_child(0);
    assert_element_name(head.clone(), "head");

    let title = head.get_child(1);
    assert_element_name(title.clone(), "title");
    assert_text_contents(title.get_child(0), "Test Document");

    let style = head.get_child(3);
    assert_element_name(style.clone(), "style");
    assert_text_contents(style.get_child(0), "* {padding: 0}");

    let script = head.get_child(5);
    assert_element_name(script.clone(), "script");
    assert_text_contents(script.get_child(0), "console.log('Hello, world')");

    let body = html.get_child(2);
    assert_element_name(body.clone(), "body");

    let h1 = body.get_child(1);
    assert_element_name(h1.clone(), "h1");
    assert_text_contents(h1.get_child(0), "Test Heading");

    let p = body.get_child(3);
    assert_element_name(p.clone(), "p");
    assert_text_contents(p.get_child(0), "Test Paragraph");
}
