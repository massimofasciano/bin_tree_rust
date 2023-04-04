use std::ops::Deref;

#[derive(Debug,Clone,PartialEq)]
pub enum BinTree<Item> {
    Empty,
    Branch(Item,Box<BinTree<Item>>,Box<BinTree<Item>>),
}

impl<Item> BinTree<Item> {
    pub fn branch(item : Item, left: BinTree<Item>, right: BinTree<Item>) -> Self {
        BinTree::Branch(item, Box::new(left), Box::new(right))
    }
    pub fn leaf(item : Item) -> Self {
        Self::branch(item, Self::empty(), Self::empty())
    }
    pub fn empty() -> Self {
        BinTree::Empty
    }
    pub fn is_branch(&self) -> bool {
        match self {
            BinTree::Branch(_,_,_) => !self.is_leaf(),
            _ => false,
        }
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            BinTree::Branch(_,left,right) => 
                left.is_empty() && right.is_empty(),
            _ => false,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            BinTree::Empty => true,
            _ => false,
        }
    }
}

impl<Item> Default for BinTree<Item> {
    fn default() -> Self {
        Self::empty()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

impl<Item : std::fmt::Display> std::fmt::Display for BinTree<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_line(f)
    }
}

impl<Item> BinTree<Item> {
    pub fn write_line(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
        where Item : std::fmt::Display 
    {
        match self {
            BinTree::Empty => { 
                write!(f,"()")
            },
            BinTree::Branch(item, left, right) => {
                write!(f,"(")?;
                if !left.is_empty() {
                    left.write_line(f)?;
                    write!(f," <= ")?;
                }
                write!(f,"{}",item)?;
                if !right.is_empty() {
                    write!(f," => ")?;
                    right.write_line(f)?;
                }
                write!(f,")")
            }
        }        
    }
    pub fn pretty_write(&self, f: &mut std::fmt::Formatter<'_>, tab: &str) -> std::fmt::Result
        where Item : std::fmt::Display 
    {
        self.pretty_write_indent(f, tab, 0)
    }
    fn pretty_write_indent(&self, f: &mut std::fmt::Formatter<'_>, tab : &str, indent : usize) -> std::fmt::Result
        where Item : std::fmt::Display 
    {
        match self {
            BinTree::Empty => { 
                write!(f,"{}{}\n",tab.repeat(indent),"@")
            },
            BinTree::Branch(item, left, right) => {
                right.pretty_write_indent(f, tab, indent+1)?;
                write!(f,"{}{}\n",tab.repeat(indent),item)?;
                left.pretty_write_indent(f, tab, indent+1)
            }
        }        
    }
}

pub enum FormattedBinTreeType<'a> {
    PrettyIndent(&'a str),
    Line,
}

pub struct FormattedBinTree<'a, T> {
    inner: &'a BinTree<T>,
    format: FormattedBinTreeType<'a>,
}

impl<'a,T : std::fmt::Display> std::fmt::Display for FormattedBinTree<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.format {
            FormattedBinTreeType::Line => self.inner.write_line(f),
            FormattedBinTreeType::PrettyIndent(tab) => self.inner.pretty_write(f,tab),
        }
    }
}

impl<'a,T> FormattedBinTree<'a,T> {
    pub fn new(t : &'a BinTree<T>, fmt: FormattedBinTreeType<'a>) -> Self {
        Self { inner: t, format: fmt }
    }
}

impl<'a,T > Deref for FormattedBinTree<'a,T> {
    type Target = BinTree<T>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

enum BinTreeIntoIterStackItem<T> {
    Item(T),
    Tree(BinTree<T>),
}

pub struct BinTreeIntoIter<T> {
    stack: Vec<BinTreeIntoIterStackItem<T>>
}

impl<T> IntoIterator for BinTree<T> {
    type IntoIter = BinTreeIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        BinTreeIntoIter { stack: vec![BinTreeIntoIterStackItem::Tree(self)] }
    }
}

impl<T> Iterator for BinTreeIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(BinTreeIntoIterStackItem::Item(item)) => Some(item),
            Some(BinTreeIntoIterStackItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIntoIterStackItem::Tree(BinTree::Branch(item, left, right))) => {
                self.stack.push(BinTreeIntoIterStackItem::Tree(*right));
                self.stack.push(BinTreeIntoIterStackItem::Item(item));
                self.stack.push(BinTreeIntoIterStackItem::Tree(*left));
                self.next()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

enum BinTreeIterStackItem<'a,T> {
    Item(&'a T),
    Tree(&'a BinTree<T>),
}

pub struct BinTreeIter<'a, T> {
    stack: Vec<BinTreeIterStackItem<'a,T>>
}

impl<'a, T> BinTree<T> {
    pub fn iter(&'a self) -> BinTreeIter<'a, T> {
        BinTreeIter { stack: vec![BinTreeIterStackItem::Tree(self)] }
    }
}

impl<'a,T> Iterator for BinTreeIter<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(BinTreeIterStackItem::Item(item)) => Some(item),
            Some(BinTreeIterStackItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIterStackItem::Tree(BinTree::Branch(item, left, right))) => {
                self.stack.push(BinTreeIterStackItem::Tree(right.as_ref()));
                self.stack.push(BinTreeIterStackItem::Item(item));
                self.stack.push(BinTreeIterStackItem::Tree(left.as_ref()));
                self.next()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

enum BinTreeIterMutStackItem<'a,T> {
    Item(&'a mut T),
    Tree(&'a mut BinTree<T>),
}

pub struct BinTreeIterMut<'a, T> {
    stack: Vec<BinTreeIterMutStackItem<'a,T>>
}

impl<'a, T> BinTree<T> {
    pub fn iter_mut(&'a mut self) -> BinTreeIterMut<'a, T> {
        BinTreeIterMut { stack: vec![BinTreeIterMutStackItem::Tree(self)] }
    }
}

impl<'a,T> Iterator for BinTreeIterMut<'a,T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(BinTreeIterMutStackItem::Item(item)) => Some(item),
            Some(BinTreeIterMutStackItem::Tree(BinTree::Empty)) => self.next(),
            Some(BinTreeIterMutStackItem::Tree(BinTree::Branch(item, left, right))) => {
                self.stack.push(BinTreeIterMutStackItem::Tree(right.as_mut()));
                self.stack.push(BinTreeIterMutStackItem::Item(item));
                self.stack.push(BinTreeIterMutStackItem::Tree(left.as_mut()));
                self.next()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

impl<T> From<()> for BinTree<T> {
    fn from(_: ()) -> Self {
        Self::empty()
    }
}

pub fn tree<T>(item: T, left: impl Into<BinTree<T>>, right: impl Into<BinTree<T>>) -> BinTree<T> {
    BinTree::branch(item, left.into(), right.into())
}

pub fn leaf<T>(item: T) -> BinTree<T> {
    BinTree::leaf(item)
}

///////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod test {
    use crate::{BinTree, tree, leaf};

    #[test]
    fn eq_test() {
        let t1 = 
            BinTree::branch(1,
                BinTree::branch(2,
                    BinTree::leaf(3),
                    BinTree::empty(),
                ),
                BinTree::branch(4,
                    BinTree::empty(),
                    BinTree::branch(5, 
                        BinTree::leaf(6), 
                        BinTree::empty()
                    )
                )
            );
        let t2 = 
            tree(1,
                tree(2,
                    leaf(3),
                    ()
                ),
                tree(4,
                    (),
                    tree(5,
                        leaf(6),
                        ()
            )));
        assert_eq!(t1,t2);
    }

    #[test]
    fn iter_mut_test() {
        let mut t = 
            tree(1,
                tree(2,
                    tree(3,
                        leaf(4),
                        ()
                    ),
                    ()
                ),
                leaf(5)
            );
        t.iter_mut().for_each(|i| {
            if *i % 2 == 1 { *i += 10 }
        });
        assert_eq!(t.to_string(),"((((4) <= 13) <= 2) <= 11 => (15))");
    }
}