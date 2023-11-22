use typst::doc::{Document, FrameItem};
use typst::eval::Datetime;

#[tracing::instrument(skip_all)]
pub fn blog(
    document: &Document,
    ident: Option<&str>,
    timestamp: Option<Datetime>,
) -> Vec<u8> {
    let mut ctx = BlogContext::new(document);
    
    for frame in &document.pages {
        for &(pos, ref item) in frame.items() {
            match item {
                FrameItem::Text(text) => {
                    println!("{}", text.text.as_str())
                }
                _ => {}
            }
        }
    }

    vec! [0]
}

/// Context for exporting a whole blog / document
struct BlogContext<'a> {
    /// the document for the blog that we're currently exporting
    document: &'a Document
}

impl<'a> BlogContext<'a> {
    pub fn new(document: &'a Document) -> Self {
        Self {
            document,
        }
    }
}