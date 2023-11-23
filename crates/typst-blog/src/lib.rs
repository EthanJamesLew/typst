use std::path::PathBuf;

use typst::diag::StrResult;
use typst::doc::{Document, FrameItem};
use typst::eval::Datetime;

/// Blog Struct (contains the generated webpages and links to assets)
#[derive(Default)]
pub struct Blog {
}

impl Blog {
    pub fn write(&self, out_path: &PathBuf) -> StrResult<()> {
        Ok(())
    }
}

#[tracing::instrument(skip_all)]
pub fn blog(
    document: &Document,
    ident: Option<&str>,
    timestamp: Option<Datetime>,
) -> Blog {
    let mut ctx = BlogContext::new(document);
    
    //for frame in &document.pages {
    //    for &(pos, ref item) in frame.items() {
    //        match item {
    //            FrameItem::Group(group) => {
    //                println!("group {:?}", group.frame)
    //            },
    //            FrameItem::Text(text) => {
    //                println!("{}", text.text.as_str())
    //            }
    //            _ => {}
    //        }
    //    }
    //}


    Blog::default()
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
