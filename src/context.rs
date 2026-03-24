use std::{error::Error, fs, io, path::Path};

use boa_engine::Context as BoaContext;
use html5ever::{parse_document, tendril::TendrilSink};
use markup5ever_rcdom::RcDom;

use crate::dom::node::Node;

pub struct BrowserContext {
    pub js: BoaContext,
    pub dom: Node,
}

impl BrowserContext {
    pub fn new() -> Option<Self> {
        let mut ctx = BoaContext::default();

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
        .ok()?;

        Some(Self {
            js: ctx,
            dom: Node::default(),
        })
    }

    pub fn load_document_from_file(&mut self, path: &Path) -> io::Result<()> {
        let text = fs::read_to_string(path)?;

        self.dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut text.as_bytes())?
            .document
            .into();

        Ok(())
    }

    pub fn load_document_from_network(&mut self, url: String) -> Result<(), Box<dyn Error>> {
        let text = reqwest::blocking::get(url)?.text()?;

        self.dom = parse_document(RcDom::default(), Default::default())
            .from_utf8()
            .read_from(&mut text.as_bytes())?
            .document
            .into();

        Ok(())
    }
}
