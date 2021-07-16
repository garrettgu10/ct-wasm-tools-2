use crate::ast::{self, kw};
use crate::parser::{Parse, Parser, Result};

/// A nested WebAssembly instance to be created as part of a module.
#[derive(Debug)]
pub struct Instance<'a> {
    /// Where this `instance` was defined.
    pub span: ast::Span,
    /// An identifier that this instance is resolved with (optionally) for name
    /// resolution.
    pub id: Option<ast::Id<'a>>,
    /// If present, inline export annotations which indicate names this
    /// definition should be exported under.
    pub exports: ast::InlineExport<'a>,
    /// What kind of instance this is, be it an inline-defined or imported one.
    pub kind: InstanceKind<'a>,
}

/// Possible ways to define a instance in the text format.
#[derive(Debug)]
pub enum InstanceKind<'a> {
    /// An instance which is actually defined as an import, such as:
    Import {
        /// Where we're importing from
        import: ast::InlineImport<'a>,
        /// The type that this instance will have.
        ty: ast::TypeUse<'a, ast::InstanceType<'a>>,
    },

    /// Instances whose instantiation is defined inline.
    Inline {
        /// Module that we're instantiating
        module: ast::Index<'a>,
        /// Items used to instantiate the instance
        items: Vec<ast::ExportKind<'a>>,
    },
}

impl<'a> Parse<'a> for Instance<'a> {
    fn parse(parser: Parser<'a>) -> Result<Self> {
        let span = parser.parse::<kw::instance>()?.0;
        let id = parser.parse()?;
        let exports = parser.parse()?;

        let kind = if let Some(import) = parser.parse()? {
            InstanceKind::Import {
                import,
                ty: parser.parse()?,
            }
        } else {
            parser.parens(|p| {
                p.parse::<kw::instantiate>()?;
                let module = p.parse()?;
                let mut items = Vec::new();
                while !p.is_empty() {
                    items.push(p.parens(|p| p.parse())?);
                }
                Ok(InstanceKind::Inline { module, items })
            })?
        };

        Ok(Instance {
            span,
            id,
            exports,
            kind,
        })
    }
}
