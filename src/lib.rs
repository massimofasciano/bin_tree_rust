use std::ops::Deref;

#[derive(Debug,Clone,PartialEq)]
pub enum Tree<Item> {
    Empty,
    Branch(Item,Box<Tree<Item>>,Box<Tree<Item>>),
}

impl<Item> Tree<Item> {
    pub fn branch(item : Item, left: Tree<Item>, right: Tree<Item>) -> Self {
        Tree::Branch(item, Box::new(left), Box::new(right))
    }
    pub fn leaf(item : Item) -> Self {
        Self::branch(item, Self::empty(), Self::empty())
    }
    pub fn empty() -> Self {
        Tree::Empty
    }
    pub fn is_branch(&self) -> bool {
        match self {
            Tree::Branch(_,_,_) => !self.is_leaf(),
            _ => false,
        }
    }
    pub fn is_leaf(&self) -> bool {
        match self {
            Tree::Branch(_,left,right) => 
                left.is_empty() && right.is_empty(),
            _ => false,
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Tree::Empty => true,
            _ => false,
        }
    }
}

impl<Item> Default for Tree<Item> {
    fn default() -> Self {
        Self::empty()
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

impl<Item : std::fmt::Display> std::fmt::Display for Tree<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_line(f)
    }
}

impl<Item> Tree<Item> {
    pub fn write_line(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
        where Item : std::fmt::Display 
    {
        match self {
            Tree::Empty => { 
                write!(f,"()")
            },
            Tree::Branch(item, left, right) => {
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
            Tree::Empty => { 
                write!(f,"{}{}\n",tab.repeat(indent),"@")
            },
            Tree::Branch(item, left, right) => {
                right.pretty_write_indent(f, tab, indent+1)?;
                write!(f,"{}{}\n",tab.repeat(indent),item)?;
                left.pretty_write_indent(f, tab, indent+1)
            }
        }        
    }
}

pub enum FormattedTreeType<'a> {
    PrettyIndent(&'a str),
    Line,
}

pub struct FormattedTree<'a, T> {
    inner: &'a Tree<T>,
    format: FormattedTreeType<'a>,
}

impl<'a,T : std::fmt::Display> std::fmt::Display for FormattedTree<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.format {
            FormattedTreeType::Line => self.inner.write_line(f),
            FormattedTreeType::PrettyIndent(tab) => self.inner.pretty_write(f,tab),
        }
    }
}

impl<'a,T> FormattedTree<'a,T> {
    pub fn new(t : &'a Tree<T>, fmt: FormattedTreeType<'a>) -> Self {
        Self { inner: t, format: fmt }
    }
}

impl<'a,T > Deref for FormattedTree<'a,T> {
    type Target = Tree<T>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

enum TreeIntoIterStackItem<T> {
    Item(T),
    Tree(Tree<T>),
}

pub struct TreeIntoIter<T> {
    stack: Vec<TreeIntoIterStackItem<T>>
}

impl<T> IntoIterator for Tree<T> {
    type IntoIter = TreeIntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        TreeIntoIter { stack: vec![TreeIntoIterStackItem::Tree(self)] }
    }
}

impl<T> Iterator for TreeIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(TreeIntoIterStackItem::Item(item)) => Some(item),
            Some(TreeIntoIterStackItem::Tree(Tree::Empty)) => self.next(),
            Some(TreeIntoIterStackItem::Tree(Tree::Branch(item, left, right))) => {
                self.stack.push(TreeIntoIterStackItem::Tree(*right));
                self.stack.push(TreeIntoIterStackItem::Item(item));
                self.stack.push(TreeIntoIterStackItem::Tree(*left));
                self.next()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

enum TreeIterStackItem<'a,T> {
    Item(&'a T),
    Tree(&'a Tree<T>),
}

pub struct TreeIter<'a, T> {
    stack: Vec<TreeIterStackItem<'a,T>>
}

impl<'a, T> Tree<T> {
    pub fn iter(&'a self) -> TreeIter<'a, T> {
        TreeIter { stack: vec![TreeIterStackItem::Tree(self)] }
    }
}

impl<'a,T> Iterator for TreeIter<'a,T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(TreeIterStackItem::Item(item)) => Some(item),
            Some(TreeIterStackItem::Tree(Tree::Empty)) => self.next(),
            Some(TreeIterStackItem::Tree(Tree::Branch(item, left, right))) => {
                self.stack.push(TreeIterStackItem::Tree(right.as_ref()));
                self.stack.push(TreeIterStackItem::Item(item));
                self.stack.push(TreeIterStackItem::Tree(left.as_ref()));
                self.next()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

enum TreeIterMutStackItem<'a,T> {
    Item(&'a mut T),
    Tree(&'a mut Tree<T>),
}

pub struct TreeIterMut<'a, T> {
    stack: Vec<TreeIterMutStackItem<'a,T>>
}

impl<'a, T> Tree<T> {
    pub fn iter_mut(&'a mut self) -> TreeIterMut<'a, T> {
        TreeIterMut { stack: vec![TreeIterMutStackItem::Tree(self)] }
    }
}

impl<'a,T> Iterator for TreeIterMut<'a,T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        let pop = self.stack.pop();
        match pop {
            None => None,
            Some(TreeIterMutStackItem::Item(item)) => Some(item),
            Some(TreeIterMutStackItem::Tree(Tree::Empty)) => self.next(),
            Some(TreeIterMutStackItem::Tree(Tree::Branch(item, left, right))) => {
                self.stack.push(TreeIterMutStackItem::Tree(right.as_mut()));
                self.stack.push(TreeIterMutStackItem::Item(item));
                self.stack.push(TreeIterMutStackItem::Tree(left.as_mut()));
                self.next()
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

impl<T> From<()> for Tree<T> {
    fn from(_: ()) -> Self {
        Self::empty()
    }
}

pub fn tree<T>(item: T, left: impl Into<Tree<T>>, right: impl Into<Tree<T>>) -> Tree<T> {
    Tree::branch(item, left.into(), right.into())
}

pub fn leaf<T>(item: T) -> Tree<T> {
    Tree::leaf(item)
}

