use std::{fmt, sync::Arc};

#[derive(Clone)]
pub struct FunTree {
    data: Arc<FunTreeData>,
}

#[derive(Clone)]
pub struct FunTreeData {
    kind: String,
    children: Vec<FunTree>,
}

impl FunTree {
    pub fn new(kind: impl Into<String>) -> FunTreeData {
        FunTreeData { kind: kind.into(), children: Vec::new() }
    }
    pub fn kind(&self) -> &str {
        self.data.kind.as_str()
    }
    pub fn children(&self) -> impl Iterator<Item = &FunTree> + '_ {
        self.data.children.iter()
    }
    pub fn get_child(&self, index: usize) -> Option<&FunTree> {
        self.data.children.get(index)
    }
    pub fn remove_child(&self, index: usize) -> FunTree {
        let mut data = self.data.clone();
        Arc::make_mut(&mut data).children.remove(index);
        FunTree { data }
    }
    pub fn insert_child(&self, index: usize, child: FunTree) -> FunTree {
        let mut data = self.data.clone();
        Arc::make_mut(&mut data).children.insert(index, child);
        FunTree { data }
    }
    pub fn replace_child(&self, index: usize, child: FunTree) -> FunTree {
        let mut data = self.data.clone();
        Arc::make_mut(&mut data).children[index] = child;
        FunTree { data }
    }
}

impl FunTreeData {
    pub fn push(mut self, child: impl Into<FunTree>) -> FunTreeData {
        self.children.push(child.into());
        self
    }
}

impl From<FunTreeData> for FunTree {
    fn from(data: FunTreeData) -> FunTree {
        FunTree { data: Arc::new(data) }
    }
}

impl<T: Into<String>> From<T> for FunTree {
    fn from(kind: T) -> Self {
        FunTree::new(kind).into()
    }
}

impl fmt::Display for FunTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            fmt_rec(f, 0, self)
        } else {
            write!(f, "{}", self.kind())
        }
    }
}
fn fmt_rec(f: &mut fmt::Formatter<'_>, lvl: usize, tree: &FunTree) -> fmt::Result {
    writeln!(f, "{:indent$}{}", "", tree.kind(), indent = lvl * 2)?;
    for child in tree.children() {
        fmt_rec(f, lvl + 1, child)?;
    }
    Ok(())
}
impl fmt::Debug for FunTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl PartialEq for FunTree {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }
}

impl Eq for FunTree {}
