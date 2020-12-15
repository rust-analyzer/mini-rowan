use std::{fmt, mem, sync::Arc};

use crate::delta::Delta;

#[derive(Clone)]
pub struct FunTree {
    data: Arc<FunTreeData>,
}

#[derive(Clone)]
pub struct FunTreeData {
    kind: &'static str,
    len: usize,
    children: Vec<FunChild>,
}

#[derive(Clone)]
pub struct FunToken {
    kind: &'static str,
    text: String,
}

#[derive(Clone)]
pub struct FunChild {
    pub offset: usize,
    pub kind: FunChildKind,
}

#[derive(Clone)]
pub enum FunChildKind {
    Tree(FunTree),
    Token(FunToken),
}

impl FunChildKind {
    pub fn len(&self) -> usize {
        match self {
            FunChildKind::Tree(it) => it.len(),
            FunChildKind::Token(it) => it.len(),
        }
    }
    pub fn kind(&self) -> &'static str {
        match self {
            FunChildKind::Tree(it) => it.kind(),
            FunChildKind::Token(it) => it.kind(),
        }
    }
}

impl FunToken {
    pub fn new(kind: &'static str, text: impl Into<String>) -> FunToken {
        let text = text.into();
        FunToken { kind, text }
    }
    pub fn kind(&self) -> &'static str {
        self.kind
    }
    pub fn len(&self) -> usize {
        self.text.len()
    }
}

impl FunTree {
    pub fn new(kind: &'static str) -> FunTreeData {
        FunTreeData { kind: kind.into(), len: 0, children: Vec::new() }
    }
    pub fn kind(&self) -> &'static str {
        self.data.kind
    }
    pub fn len(&self) -> usize {
        self.data.len
    }
    pub fn children(&self) -> impl Iterator<Item = &FunChild> + '_ {
        self.data.children.iter()
    }
    pub fn get_child(&self, index: usize) -> Option<&FunChild> {
        self.data.children.get(index)
    }
    pub fn remove_child(&self, index: usize) -> FunTree {
        self.modify(index + 1, |children| {
            let old_child = children.remove(index);
            Delta::Sub(old_child.kind.len())
        })
    }
    pub fn insert_child(&self, index: usize, child: FunChildKind) -> FunTree {
        self.modify(index + 1, |children| {
            let len = child.len();
            let offset = children.get(0).map_or(0, |it| it.offset);
            children.insert(index, FunChild { offset, kind: child });
            Delta::Add(len)
        })
    }
    pub fn replace_child(&self, index: usize, child: FunChildKind) -> FunTree {
        self.modify(index, |children| {
            let new_len = child.len();
            let old_child = mem::replace(&mut children[index].kind, child);
            let old_len = old_child.len();
            Delta::new(old_len, new_len)
        })
    }
    fn modify(&self, index: usize, op: impl FnOnce(&mut Vec<FunChild>) -> Delta<usize>) -> FunTree {
        let mut data = self.data.clone();
        {
            let data = Arc::make_mut(&mut data);
            let delta = op(&mut data.children);
            for child in &mut data.children[index..] {
                child.offset += delta;
            }
        }
        FunTree { data }
    }
}

impl FunTreeData {
    pub fn push(mut self, child: impl Into<FunChildKind>) -> FunTreeData {
        let kind = child.into();
        let offset = self.len;
        self.len += kind.len();
        self.children.push(FunChild { offset, kind });
        self
    }
}

impl From<FunTreeData> for FunTree {
    fn from(data: FunTreeData) -> FunTree {
        FunTree { data: Arc::new(data) }
    }
}

impl From<FunTreeData> for FunChildKind {
    fn from(data: FunTreeData) -> FunChildKind {
        FunChildKind::Tree(data.into())
    }
}

impl From<FunToken> for FunChildKind {
    fn from(token: FunToken) -> FunChildKind {
        FunChildKind::Token(token)
    }
}

impl fmt::Debug for FunTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            fmt_rec(f, 0, self)
        } else {
            write!(f, "{}", self.kind())
        }
    }
}
impl fmt::Debug for FunToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.text, self.kind)
    }
}

fn fmt_rec(f: &mut fmt::Formatter<'_>, lvl: usize, tree: &FunTree) -> fmt::Result {
    writeln!(f, "{:indent$}{}", "", tree.kind(), indent = lvl * 2)?;
    for child in tree.children() {
        match &child.kind {
            FunChildKind::Tree(it) => fmt_rec(f, lvl + 1, &it)?,
            FunChildKind::Token(it) => writeln!(f, "{:indent$}{:?}", "", it, indent = lvl * 2 + 2)?,
        }
    }
    Ok(())
}
