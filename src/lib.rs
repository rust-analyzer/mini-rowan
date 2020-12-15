mod fun;
mod sll;
mod delta;

use std::{
    cell::{Cell, RefCell},
    fmt, iter,
    rc::{self, Rc},
};

pub use fun::{FunChild, FunChildKind, FunToken, FunTree, FunTreeData};

#[derive(Clone)]
pub struct SyntaxTree {
    data: Rc<SyntaxData>,
}

#[derive(Clone)]
pub struct SyntaxToken {
    data: Rc<SyntaxData>,
}

pub enum SyntaxChild {
    Tree(SyntaxTree),
    Token(SyntaxToken),
}

enum Fun {
    Tree(RefCell<FunTree>),
    Token(FunToken),
}

struct SyntaxData {
    fun: Fun,

    parent: Cell<Option<SyntaxTree>>,
    index: Cell<usize>,

    first: Cell<rc::Weak<SyntaxData>>,
    // Invariant: never null
    next: Cell<rc::Weak<SyntaxData>>,
    prev: Cell<rc::Weak<SyntaxData>>,
}

impl sll::Elem for SyntaxData {
    fn prev(&self) -> &Cell<rc::Weak<Self>> {
        &self.prev
    }
    fn next(&self) -> &Cell<rc::Weak<Self>> {
        &self.next
    }
    fn key(&self) -> &Cell<usize> {
        &self.index
    }
}

impl SyntaxChild {
    fn new(fun: FunChild, parent: SyntaxTree, index: usize) -> SyntaxChild {
        let mut token = false;
        let fun = match fun.kind {
            FunChildKind::Tree(it) => Fun::Tree(RefCell::new(it)),
            FunChildKind::Token(it) => {
                token = true;
                Fun::Token(it)
            }
        };
        let data = SyntaxData {
            fun,
            parent: Cell::new(Some(parent)),
            index: Cell::new(index),
            first: Default::default(),
            next: Default::default(),
            prev: Default::default(),
        };
        let data = Rc::new(data);
        data.next.set(Rc::downgrade(&data));
        data.prev.set(Rc::downgrade(&data));
        if token {
            SyntaxChild::Token(SyntaxToken { data })
        } else {
            SyntaxChild::Tree(SyntaxTree { data })
        }
    }
    fn data(&self) -> &Rc<SyntaxData> {
        match self {
            SyntaxChild::Tree(it) => &it.data,
            SyntaxChild::Token(it) => &it.data,
        }
    }
    fn data_mut(&mut self) -> &mut Rc<SyntaxData> {
        match self {
            SyntaxChild::Tree(it) => &mut it.data,
            SyntaxChild::Token(it) => &mut it.data,
        }
    }
}

impl SyntaxChild {
    pub fn kind(&self) -> &'static str {
        match self {
            SyntaxChild::Tree(it) => it.kind(),
            SyntaxChild::Token(it) => it.kind(),
        }
    }
    pub fn offset(&self) -> usize {
        match self {
            SyntaxChild::Tree(it) => it.offset(),
            SyntaxChild::Token(it) => it.offset(),
        }
    }
    pub fn parent(&self) -> Option<SyntaxTree> {
        match self {
            SyntaxChild::Tree(it) => it.parent(),
            SyntaxChild::Token(it) => it.parent(),
        }
    }
    pub fn next_sibling(&self) -> Option<SyntaxChild> {
        match self {
            SyntaxChild::Tree(it) => it.next_sibling(),
            SyntaxChild::Token(it) => it.next_sibling(),
        }
    }
    pub fn prev_sibling(&self) -> Option<SyntaxChild> {
        match self {
            SyntaxChild::Tree(it) => it.prev_sibling(),
            SyntaxChild::Token(it) => it.prev_sibling(),
        }
    }
}

impl SyntaxToken {
    pub fn kind(&self) -> &'static str {
        self.data.kind()
    }
    pub fn offset(&self) -> usize {
        self.data.offset()
    }
    pub fn parent(&self) -> Option<SyntaxTree> {
        self.data.parent()
    }
    pub fn next_sibling(&self) -> Option<SyntaxChild> {
        self.data.next_sibling()
    }
    pub fn prev_sibling(&self) -> Option<SyntaxChild> {
        self.data.prev_sibling()
    }
}

impl SyntaxTree {
    fn new(fun: FunTree) -> SyntaxTree {
        let data = SyntaxData {
            fun: Fun::Tree(RefCell::new(fun)),
            parent: Cell::new(None),
            index: Cell::new(0),
            first: Default::default(),
            next: Default::default(),
            prev: Default::default(),
        };
        let data = Rc::new(data);
        data.next.set(Rc::downgrade(&data));
        data.prev.set(Rc::downgrade(&data));
        SyntaxTree { data }
    }
    pub fn kind(&self) -> &'static str {
        self.data.kind()
    }
    pub fn offset(&self) -> usize {
        self.data.offset()
    }
    pub fn parent(&self) -> Option<SyntaxTree> {
        self.data.parent()
    }
    pub fn first_child(&self) -> Option<SyntaxChild> {
        self.get_child(0)
    }
    pub fn next_sibling(&self) -> Option<SyntaxChild> {
        self.data.next_sibling()
    }
    pub fn prev_sibling(&self) -> Option<SyntaxChild> {
        self.data.prev_sibling()
    }
    fn get_child(&self, index: usize) -> Option<SyntaxChild> {
        let fun = self.fun().borrow().get_child(index).cloned()?;
        let mut res = SyntaxChild::new(fun, self.clone(), index);
        sll::link(&self.data.first, res.data_mut());
        Some(res)
    }

    pub fn children(&self) -> impl Iterator<Item = SyntaxChild> {
        iter::successors(self.first_child(), |it| it.next_sibling())
    }
    pub fn find(&self, kind: &str) -> Option<SyntaxChild> {
        self.children().find(|it| it.kind() == kind)
    }

    //     pub fn insert_child(&self, index: usize, mut child: SyntaxTree) {
    //         assert!(child.parent().is_none());
    //         let weak = self.data.first.take();
    //         let first = weak.upgrade();
    //         self.data.first.set(weak);
    //         if let Some(first) = first {
    //             sll::adjust(&first, index, 1);
    //         }
    //         sll::link(&self.data.first, &mut child.data);

    //         let fun = self.data.fun.borrow().insert_child(index, child.data.fun.borrow().clone());
    //         self.replace_fun(fun)
    //     }
    //     pub fn detach(&self) {
    //         if let Some(parent) = self.parent() {
    //             let fun = parent.data.fun.borrow().remove_child(self.data.index.get());
    //             parent.replace_fun(fun);
    //         }
    //         sll::adjust(&self.data, self.data.index.get() + 1, -1);
    //         self.unlink();
    //     }
    //     fn replace_fun(&self, mut fun: FunTree) {
    //         let mut node = self.clone();
    //         loop {
    //             *node.data.fun.borrow_mut() = fun.clone();
    //             match node.parent() {
    //                 Some(parent) => {
    //                     fun = parent.data.fun.borrow().replace_child(node.data.index.get(), fun);
    //                     node = parent
    //                 }
    //                 None => return,
    //             }
    //         }
    //     }
    //     fn unlink(&self) {
    //         let dummy;
    //         let parent = self.data.parent.take();
    //         let head = match parent.as_ref() {
    //             Some(it) => &it.data.first,
    //             None => {
    //                 dummy = Cell::new(rc::Weak::new());
    //                 &dummy
    //             }
    //         };
    //         sll::unlink(head, &self.data);
    //         self.data.index.set(0);
    //     }

    fn fun(&self) -> &RefCell<FunTree> {
        match &self.data.fun {
            Fun::Tree(it) => it,
            Fun::Token(_) => unreachable!(),
        }
    }
}

// impl Drop for SyntaxTree {
//     fn drop(&mut self) {
//         if Rc::strong_count(&self.data) == 1 {
//             assert!(self.data.first.take().strong_count() == 0);
//             self.unlink()
//         }
//     }
// }

impl From<FunTree> for SyntaxTree {
    fn from(fun: FunTree) -> Self {
        SyntaxTree::new(fun)
    }
}

impl fmt::Debug for SyntaxTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&*self.fun().borrow(), f)
    }
}

impl PartialEq for SyntaxTree {
    fn eq(&self, other: &SyntaxTree) -> bool {
        Rc::ptr_eq(&self.data, &other.data)
    }
}

impl Eq for SyntaxTree {}

impl SyntaxData {
    fn kind(&self) -> &'static str {
        match &self.fun {
            Fun::Tree(it) => it.borrow().kind(),
            Fun::Token(it) => it.kind(),
        }
    }
    fn offset(&self) -> usize {
        let mut offset = 0;
        let mut curr = self.clone();
        if let Some(parent) = curr.parent() {
            let idx = self.index.get();
            offset += parent.offset();
            offset += parent.fun().borrow().get_child(idx).unwrap().offset;
        }
        offset
    }
    fn parent(&self) -> Option<SyntaxTree> {
        let ret = self.parent.take();
        self.parent.set(ret.clone());
        ret
    }
    fn next_sibling(&self) -> Option<SyntaxChild> {
        let parent = self.parent()?;
        let index = self.index.get() + 1;
        parent.get_child(index)
    }
    fn prev_sibling(&self) -> Option<SyntaxChild> {
        let parent = self.parent()?;
        let index = self.index.get().checked_sub(1)?;
        parent.get_child(index)
    }
}
