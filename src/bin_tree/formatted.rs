use crate::BinTree;
use std::ops::Deref;

/// different kinds of formatting for the tree
pub enum FormattedBinTreeType<'a> {
    PrettyIndent(&'a str),
    Line,
}

/// a container for a formatted tree
pub struct FormattedBinTree<'a, T> {
    inner: &'a BinTree<T>,
    format: FormattedBinTreeType<'a>,
}

impl<'a,T : std::fmt::Debug> std::fmt::Display for FormattedBinTree<'a,T> {
    /// display a formatted tree according to the internal format field
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.format {
            FormattedBinTreeType::Line => self.inner.write_line(f),
            FormattedBinTreeType::PrettyIndent(tab) => self.inner.pretty_write(f,tab),
        }
    }
}

impl<'a,T> FormattedBinTree<'a,T> {
    /// associate a BinTree with a format
    pub fn new(t : &'a BinTree<T>, fmt: FormattedBinTreeType<'a>) -> Self {
        Self { inner: t, format: fmt }
    }
}

impl<'a,T > Deref for FormattedBinTree<'a,T> {
    type Target = BinTree<T>;

    /// returns a ref to the inner tree
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
