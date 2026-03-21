use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::RcDom;

use crate::dom::node::{Handle, Node, NodeData};

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

fn parse_tree() -> Handle {
    Node::from_rcdom(
        parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut TEST_DOCUMENT.as_bytes())
            .unwrap()
            .document,
    )
}

fn assert_element_name(node: Handle, expected_name: &str) {
    match &node.borrow().data {
        NodeData::Element { name, .. } => assert_eq!(name, expected_name),
        data @ _ => panic!("expected an element, got {:?}", data),
    }
}

fn assert_text_contents(node: Handle, expected_text: &str) {
    match &node.borrow().data {
        NodeData::Text(contents) => assert_eq!(contents, expected_text),
        data @ _ => panic!("expected a text node, got {:?}", data),
    }
}

#[test]
fn test_conversion() {
    let tree = parse_tree();
    dbg!(&tree);

    assert!(
        matches!(tree.borrow().data, NodeData::Document),
        "Top-level node should be a document node"
    );

    let doctype = tree.borrow().children[0].clone();
    assert!(
        matches!(doctype.borrow().data, NodeData::Doctype { .. }),
        "First child of the document should be a doctype node"
    );

    let html = tree.borrow().children[1].clone();
    assert_element_name(html.clone(), "html");

    let head = html.borrow().children[0].clone();
    assert_element_name(head.clone(), "head");

    let title = head.borrow().children[1].clone();
    assert_element_name(title.clone(), "title");
    assert_text_contents(title.borrow().children[0].clone(), "Test Document");

    let style = head.borrow().children[3].clone();
    assert_element_name(style.clone(), "style");
    assert_text_contents(style.borrow().children[0].clone(), "* {padding: 0}");

    let script = head.borrow().children[5].clone();
    assert_element_name(script.clone(), "script");
    assert_text_contents(
        script.borrow().children[0].clone(),
        "console.log('Hello, world')",
    );

    let body = html.borrow().children[2].clone();
    assert_element_name(body.clone(), "body");

    let h1 = body.borrow().children[1].clone();
    assert_element_name(h1.clone(), "h1");
    assert_text_contents(h1.borrow().children[0].clone(), "Test Heading");

    let p = body.borrow().children[3].clone();
    assert_element_name(p.clone(), "p");
    assert_text_contents(p.borrow().children[0].clone(), "Test Paragraph");
}
