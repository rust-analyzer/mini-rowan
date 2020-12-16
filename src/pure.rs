use std::{fmt, mem, sync::Arc};

use crate::delta::Delta;

#[derive(Clone)]
pub struct PureTree {
    data: Arc<PureTreeData>,
}

#[derive(Clone)]
pub struct PureTreeData {
    kind: &'static str,
    text_len: usize,
    children: Vec<PureChild>,
}

#[derive(Clone)]
pub struct PureToken {
    kind: &'static str,
    text: String,
}

#[derive(Clone, Debug)]
pub struct PureChild {
    pub offset: usize,
    pub kind: PureChildKind,
}

#[derive(Clone, Debug)]
pub enum PureChildKind {
    Tree(PureTree),
    Token(PureToken),
}

impl PureChildKind {
    pub fn text_len(&self) -> usize {
        match self {
            PureChildKind::Tree(it) => it.text_len(),
            PureChildKind::Token(it) => it.text_len(),
        }
    }
    pub fn kind(&self) -> &'static str {
        match self {
            PureChildKind::Tree(it) => it.kind(),
            PureChildKind::Token(it) => it.kind(),
        }
    }
}

impl PureToken {
    pub fn new(kind: &'static str, text: impl Into<String>) -> PureToken {
        let text = text.into();
        PureToken { kind, text }
    }
    pub fn kind(&self) -> &'static str {
        self.kind
    }
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
    pub fn text_len(&self) -> usize {
        self.text.len()
    }
}

impl PureTree {
    pub fn new(kind: &'static str) -> PureTreeData {
        PureTreeData { kind: kind.into(), text_len: 0, children: Vec::new() }
    }
    pub fn kind(&self) -> &'static str {
        self.data.kind
    }
    pub fn text_len(&self) -> usize {
        self.data.text_len
    }
    pub fn children(&self) -> impl Iterator<Item = &PureChild> + '_ {
        self.data.children.iter()
    }
    pub fn get_child(&self, index: usize) -> Option<&PureChild> {
        self.data.children.get(index)
    }
    pub fn remove_child(&self, index: usize) -> PureTree {
        self.modify(index, |children| {
            let old_child = children.remove(index);
            Delta::Sub(old_child.kind.text_len())
        })
    }
    pub fn insert_child(&self, index: usize, child: PureChildKind) -> PureTree {
        self.modify(index + 1, |children| {
            let len = child.text_len();
            let offset = children.get(0).map_or(0, |it| it.offset);
            children.insert(index, PureChild { offset, kind: child });
            Delta::Add(len)
        })
    }
    pub fn replace_child(&self, index: usize, child: PureChildKind) -> PureTree {
        self.modify(index + 1, |children| {
            let new_len = child.text_len();
            let old_child = mem::replace(&mut children[index].kind, child);
            let old_len = old_child.text_len();
            Delta::new(old_len, new_len)
        })
    }
    fn modify(
        &self,
        index: usize,
        op: impl FnOnce(&mut Vec<PureChild>) -> Delta<usize>,
    ) -> PureTree {
        let mut data = self.data.clone();
        {
            let data = Arc::make_mut(&mut data);
            let delta = op(&mut data.children);
            for child in &mut data.children[index..] {
                child.offset += delta;
            }
            data.text_len += delta;
        }
        PureTree { data }
    }
}

impl PureTreeData {
    pub fn push(mut self, child: impl Into<PureChildKind>) -> PureTreeData {
        let kind = child.into();
        let offset = self.text_len;
        self.text_len += kind.text_len();
        self.children.push(PureChild { offset, kind });
        self
    }
}

impl From<PureTreeData> for PureTree {
    fn from(data: PureTreeData) -> PureTree {
        PureTree { data: Arc::new(data) }
    }
}

impl From<PureTreeData> for PureChildKind {
    fn from(data: PureTreeData) -> PureChildKind {
        PureChildKind::Tree(data.into())
    }
}

impl From<PureToken> for PureChildKind {
    fn from(token: PureToken) -> PureChildKind {
        PureChildKind::Token(token)
    }
}

impl From<PureTree> for PureChildKind {
    fn from(token: PureTree) -> PureChildKind {
        PureChildKind::Tree(token)
    }
}

impl fmt::Debug for PureTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            fmt_rec(f, 0, self)
        } else {
            write!(f, "{}", self.kind())
        }
    }
}
impl fmt::Debug for PureToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.text, self.kind)
    }
}

fn fmt_rec(f: &mut fmt::Formatter<'_>, lvl: usize, tree: &PureTree) -> fmt::Result {
    writeln!(f, "{:indent$}{}", "", tree.kind(), indent = lvl * 2)?;
    for child in tree.children() {
        match &child.kind {
            PureChildKind::Tree(it) => fmt_rec(f, lvl + 1, &it)?,
            PureChildKind::Token(it) => {
                writeln!(f, "{:indent$}{:?}", "", it, indent = lvl * 2 + 2)?
            }
        }
    }
    Ok(())
}
