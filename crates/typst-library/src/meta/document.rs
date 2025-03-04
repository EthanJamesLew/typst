use comemo::Prehashed;
use typst::eval::Datetime;

use crate::layout::{LayoutRoot, PageElem};
use crate::meta::ManualPageCounter;
use crate::prelude::*;

/// The root element of a document and its metadata.
///
/// All documents are automatically wrapped in a `document` element. You cannot
/// create a document element yourself. This function is only used with
/// [set rules]($styling/#set-rules) to specify document metadata. Such a set
/// rule must appear before any of the document's contents.
///
/// ```example
/// #set document(title: [Hello])
///
/// This has no visible output, but
/// embeds metadata into the PDF!
/// ```
///
/// Note that metadata set with this function is not rendered within the
/// document. Instead, it is embedded in the compiled PDF file.
#[elem(Construct, LayoutRoot)]
pub struct DocumentElem {
    /// The document's title. This is often rendered as the title of the
    /// PDF viewer window.
    ///
    /// While this can be arbitrary content, PDF viewers only support plain text
    /// titles, so the conversion might be lossy.
    pub title: Option<Content>,

    /// The document's authors.
    pub author: Author,

    /// The document's keywords.
    pub keywords: Keywords,

    /// The document's creation date.
    ///
    /// If this is `{auto}` (default), Typst uses the current date and time.
    /// Setting it to `{none}` prevents Typst from embedding any creation date
    /// into the PDF metadata.
    ///
    /// The year component must be at least zero in order to be embedded into a
    /// PDF.
    pub date: Smart<Option<Datetime>>,

    /// The page runs.
    #[variadic]
    pub children: Vec<Prehashed<Content>>,
}

impl Construct for DocumentElem {
    fn construct(_: &mut Vm, args: &mut Args) -> SourceResult<Content> {
        bail!(args.span, "can only be used in set rules")
    }
}

impl LayoutRoot for DocumentElem {
    /// Layout the document into a sequence of frames, one per page.
    #[tracing::instrument(name = "DocumentElem::layout_root", skip_all)]
    fn layout_root(&self, vt: &mut Vt, styles: StyleChain) -> SourceResult<Document> {
        tracing::info!("Document layout");

        let mut pages = vec![];
        let mut page_counter = ManualPageCounter::new();

        let children = self.children();
        let mut iter = children.iter().map(|c| &**c).peekable();

        while let Some(mut child) = iter.next() {
            let outer = styles;
            let mut styles = styles;
            if let Some((elem, local)) = child.to_styled() {
                styles = outer.chain(local);
                child = elem;
            }

            if let Some(page) = child.to::<PageElem>() {
                let extend_to = iter.peek().and_then(|&next| {
                    next.to_styled()
                        .map_or(next, |(elem, _)| elem)
                        .to::<PageElem>()?
                        .clear_to(styles)
                });
                let fragment = page.layout(vt, styles, &mut page_counter, extend_to)?;
                pages.extend(fragment);
            } else {
                bail!(child.span(), "unexpected document child");
            }
        }

        Ok(Document {
            pages,
            title: self.title(styles).map(|content| content.plain_text()),
            author: self.author(styles).0,
            keywords: self.keywords(styles).0,
            date: self.date(styles),
        })
    }
}

/// A list of authors.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Author(Vec<EcoString>);

cast! {
    Author,
    self => self.0.into_value(),
    v: EcoString => Self(vec![v]),
    v: Array => Self(v.into_iter().map(Value::cast).collect::<StrResult<_>>()?),
}

/// A list of keywords.
#[derive(Debug, Default, Clone, PartialEq, Hash)]
pub struct Keywords(Vec<EcoString>);

cast! {
    Keywords,
    self => self.0.into_value(),
    v: EcoString => Self(vec![v]),
    v: Array => Self(v.into_iter().map(Value::cast).collect::<StrResult<_>>()?),
}
