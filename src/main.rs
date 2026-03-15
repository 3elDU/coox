mod document;

use std::path::Path;

use boa_engine::{Context, Source};
use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::{Handle, NodeData, RcDom};

use crate::document::Document;

fn find_element_by_tag_name(node: &Handle, tag_name: &str) -> Option<Handle> {
    if let NodeData::Element { name, .. } = &node.data {
        if &name.local == tag_name {
            return Some(node.clone());
        }
    }

    let children = node.children.borrow();
    if !children.is_empty() {
        for child in children.iter() {
            let child = find_element_by_tag_name(child, tag_name);
            if let Some(child) = child {
                return Some(child.clone());
            }
        }
    }

    None
}

fn get_text_content(node: &Handle) -> String {
    let mut joined = String::new();

    if let NodeData::Text { contents } = &node.data {
        joined += &contents.borrow();
    }

    for child in node.children.borrow().iter() {
        if let NodeData::Text { contents } = &child.data {
            joined += &contents.borrow();
        }
    }

    joined
}

fn main() {
    let path = Path::new("./test/index.html");
    let html_string = std::fs::read_to_string(path).unwrap();
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html_string.as_bytes())
        .unwrap();

    // try to find the script tag
    let script =
        find_element_by_tag_name(&dom.document, "script").expect("script element not found");
    let text = get_text_content(&script);

    println!("Script contents:");
    println!("{}", text);

    let mut ctx = Context::default();

    boa_runtime::register(
        (
            boa_runtime::extensions::ConsoleExtension::default(),
            boa_runtime::extensions::FetchExtension(
                boa_runtime::fetch::BlockingReqwestFetcher::default(),
            ),
        ),
        None,
        &mut ctx,
    )
    .unwrap();

    Document::init(&mut ctx, dom.document.clone()).unwrap();

    let result = ctx.eval(Source::from_bytes(&text)).unwrap();
    dbg!(result);
}
