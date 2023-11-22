use typst::doc::Document;
use typst::eval::Datetime;

#[tracing::instrument(skip_all)]
pub fn blog(
    document: &Document,
    ident: Option<&str>,
    timestamp: Option<Datetime>,
) -> Vec<u8> {
    let mut _ctx = BlogContext::new(document);
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