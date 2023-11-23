use std::fs;
use std::io::Write;
use std::path::PathBuf;

use typst::diag::StrResult;
use typst::doc::{Document, FrameItem};
use typst::eval::Datetime;

/// Blog Struct (contains the generated webpages and links to assets)
#[derive(Default)]
pub struct Blog {}

impl Blog {
    /// this is the main method that generates files for the website
    pub fn write(&self, out_path: &PathBuf) -> StrResult<()> {
        // Step 1: Create/Clear the output directory
        if out_path.exists() {
            fs::remove_dir_all(out_path).map_err(|e| e.to_string())?;
        }
        fs::create_dir_all(out_path).map_err(|e| e.to_string())?;

        // Step 2: Write the HTML file
        let html_content = r#"<!DOCTYPE html>
<html>
<head>
    <title>My Blog</title>
</head>
<body>
    <h1>Welcome to My Blog</h1>
    <p>This is a placeholder for blog posts.</p>
</body>
</html>"#;

        let mut file =
            fs::File::create(out_path.join("index.html")).map_err(|e| e.to_string())?;
        file.write_all(html_content.as_bytes()).map_err(|e| e.to_string())?;

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
    document: &'a Document,
}

impl<'a> BlogContext<'a> {
    pub fn new(document: &'a Document) -> Self {
        Self { document }
    }
}
