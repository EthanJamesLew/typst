use std::fs;
use std::io::Write;
use std::num::NonZeroI32;
use std::path::PathBuf;

use typst::diag::StrResult;
use typst::doc::{Document, Frame, FrameItem, Meta, GroupItem, TextItem, Destination};
use typst::eval::Datetime;
use typst::geom::{Abs, Point, Shape, Transform};
use typst::image::Image;
use typst::geom::Size;

use build_html::{Html, HtmlPage};

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
pub fn blog(
    document: &Document,
    ident: Option<&str>,
    timestamp: Option<Datetime>,
) -> Blog {
    let mut ctx = BlogContext::new(document);

    println!("{:?}", document);
    for frame in &document.pages {
        construct_page(&mut ctx, frame);
    }

    ctx.blog
}

fn construct_page(ctx: &mut BlogContext, frame: &Frame) {
    let page_ref = ctx.alloc.bump();
    ctx.page_refs.push(page_ref);

    let mut ctx = PageContext {
        parent: ctx,
    };

    // Encode the page into the content stream.
    write_frame(&mut ctx, frame);
}

/// Encode a frame into the content stream.
pub fn write_frame(ctx: &mut PageContext, frame: &Frame) {
    for &(pos, ref item) in frame.items() {
        let x = pos.x.to_f32();
        let y = pos.y.to_f32();

        match item {
            FrameItem::Group(group) => write_group(ctx, pos, group),
            FrameItem::Text(text) => write_text(ctx, pos, text),
            FrameItem::Shape(shape, _) => write_shape(ctx, pos, shape),
            FrameItem::Image(image, size, _) => write_image(ctx, x, y, image, *size),
            FrameItem::Meta(meta, size) => match meta {
                Meta::Link(dest) => write_link(ctx, pos, dest, *size),
                Meta::Elem(_) => {}
                Meta::Hide => {}
                Meta::PageNumbering(_) => {}
                Meta::PdfPageLabel(_) => {},
                _ => panic!("{:?} meta frame item not implemented", item),
            },
            _ => panic!("{:?} frame item not implemented", item)
        }
    }
}

/// Encode a group into the content stream.
fn write_group(ctx: &mut PageContext, pos: Point, group: &GroupItem) {
    // TODO: save / restore state
    write_frame(ctx, &group.frame);
}

/// Encode a text run into the content stream.
fn write_text(ctx: &mut PageContext, pos: Point, text: &TextItem) {
    println!("text: \"{:?}\", font: {:?}, size: {:?}, color: {:?}", text.text, text.size, text.font, text.fill)
}

/// Encode a geometrical shape into the content stream.
fn write_shape(ctx: &mut PageContext, pos: Point, shape: &Shape) {
}

/// Encode a vector or raster image into the content stream.
fn write_image(ctx: &mut PageContext, x: f32, y: f32, image: &Image, size: Size) {
}

/// Save a link for later writing in the annotations dictionary.
fn write_link(ctx: &mut PageContext, pos: Point, dest: &Destination, size: Size) {
}

/// An exporter for the contents of a single page.
pub struct PageContext<'a, 'b> {
    pub(crate) parent: &'a mut BlogContext<'b>,
}

/// Context for exporting a whole blog / document
struct BlogContext<'a> {
    /// the document for the blog that we're currently exporting
    document: &'a Document,
    /// the output blog
    blog: Blog,
    /// Allocator for indirect reference IDs.
    alloc: Ref,
    /// The IDs of written pages.
    page_refs: Vec<Ref>,
}

impl<'a> BlogContext<'a> {
    pub fn new(document: &'a Document) -> Self {
        let mut alloc = Ref::new(1);
        Self { 
            document,
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