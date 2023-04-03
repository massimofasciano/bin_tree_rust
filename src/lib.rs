#[derive(Debug,Clone)]
pub enum Tree<Item> {
    Empty,
    Branch(Item,Box<Tree<Item>>,Box<Tree<Item>>),
}

impl<Item> Tree<Item> {
    pub fn new_branch(item : Item, left: Tree<Item>, right: Tree<Item>) -> Self {
        Tree::Branch(item, Box::new(left), Box::new(right))
    }
    pub fn new_item(item : Item) -> Self {
        Self::new_branch(item, Default::default(), Default::default())
    }
    pub fn new() -> Self {
        Default::default()
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Tree::Empty => true,
            _ => false,
        }
    }
    pub fn pretty_write(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
        where Item : std::fmt::Display 
    {
        self.pretty_write_indent(f, 0)
    }
    fn pretty_write_indent(&self, f: &mut std::fmt::Formatter<'_>, indent : usize) -> std::fmt::Result
        where Item : std::fmt::Display 
    {
        match self {
            Tree::Empty => { 
                write!(f,"{}{}\n","  ".repeat(indent),"@")
            },
            Tree::Branch(item, left, right) => {
                right.pretty_write_indent(f, indent+1)?;
                write!(f,"{}{}\n","  ".repeat(indent),item)?;
                left.pretty_write_indent(f, indent+1)
            }
        }        
    }
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
}

impl<Item> Default for Tree<Item> {
    fn default() -> Self {
        Tree::Empty
    }
}

impl<Item : std::fmt::Display> std::fmt::Display for Tree<Item> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_line(f)
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////

#[repr(transparent)]
pub struct PrettyFormatTree<'a, T> {
    inner: &'a Tree<T>
}

impl<'a,T : std::fmt::Display> std::fmt::Display for PrettyFormatTree<'a,T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.pretty_write(f)
    }
}

impl<'a,T> PrettyFormatTree<'a,T> {
    pub fn new(t : &'a Tree<T>) -> Self {
        Self { inner: t }
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
