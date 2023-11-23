use std::fs;
use std::io::Write;
use std::num::NonZeroI32;
use std::path::PathBuf;

use typst::diag::StrResult;
use typst::doc::{Destination, Document, Frame, FrameItem, GroupItem, Meta, TextItem};
use typst::eval::Datetime;
use typst::geom::Size;
use typst::geom::{Abs, Point, Shape, Transform};
use typst::image::Image;

use build_html::{Html, HtmlPage};
use typst::model::{Content, Element};

/// Blog Struct (contains the generated webpages and links to assets)
#[derive(Default)]
pub struct Blog {
    /// the main blog page
    pub index_page: HtmlPage,
}

impl Blog {
    /// the main method that generates files for the website
    pub fn write(&self, out_path: &PathBuf) -> StrResult<()> {
        // Step 1: Create/Clear the output directory
        if out_path.exists() {
            fs::remove_dir_all(out_path).map_err(|e| e.to_string())?;
        }
        fs::create_dir_all(out_path).map_err(|e| e.to_string())?;

        // Step 2: Write the HTML file
        let html = self.index_page.to_html_string();
        let html_content = html.as_str();

        let mut file =
            fs::File::create(out_path.join("index.html")).map_err(|e| e.to_string())?;
        file.write_all(html_content.as_bytes()).map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[tracing::instrument(skip_all)]
pub fn blog(content: &Content, ident: Option<&str>, timestamp: Option<Datetime>) -> Blog {
    let mut ctx = BlogContext::new(content);
    println!("{:?}, {:?}", content, content.elem().name());
    ctx.blog
}

/// Context for exporting a whole blog content
struct BlogContext<'a> {
    /// the content for the blog that we're currently exporting
    content: &'a Content,
    /// the output blog
    blog: Blog,
    /// Allocator for indirect reference IDs.
    alloc: Ref,
    /// The IDs of written pages.
    page_refs: Vec<Ref>,
}

impl<'a> BlogContext<'a> {
    pub fn new(content: &'a Content) -> Self {
        let mut alloc = Ref::new(1);
        Self {
            content,
            blog: Blog::default(),
            alloc,
            page_refs: vec![],
        }
    }
}

/// A reference to an indirect object.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ref(NonZeroI32);

impl Ref {
    /// Create a new indirect reference.
    ///
    /// The provided value must be greater than zero.
    ///
    /// Panics if `id` is out of the valid range.
    #[inline]
    #[track_caller]
    pub const fn new(id: i32) -> Ref {
        let option = if id > 0 { NonZeroI32::new(id) } else { None };
        match option {
            Some(val) => Self(val),
            None => panic!("indirect reference out of valid range"),
        }
    }

    /// Return the underlying number as a primitive type.
    #[inline]
    pub const fn get(self) -> i32 {
        self.0.get()
    }

    /// The next consecutive ID.
    #[inline]
    pub const fn next(self) -> Self {
        Self::new(self.get() + 1)
    }

    /// Increase this ID by one and return the old one. Useful to turn this ID
    /// into a bump allocator of sorts.
    #[inline]
    pub fn bump(&mut self) -> Self {
        let prev = *self;
        *self = self.next();
        prev
    }
}

/// Additional methods for [`Abs`].
trait AbsExt {
    /// Convert an to a number of points.
    fn to_f32(self) -> f32;
}

impl AbsExt for Abs {
    fn to_f32(self) -> f32 {
        self.to_pt() as f32
    }
}
