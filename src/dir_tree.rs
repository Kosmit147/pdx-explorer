use crate::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Directory {
    path: PathBuf,
    children: Vec<Node>,
}

impl Directory {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

#[derive(Debug)]
pub struct File {
    path: PathBuf,
}

impl File {
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[derive(Debug)]
pub enum Node {
    Directory(Directory),
    File(File),
}

impl Node {
    pub fn path(&self) -> &Path {
        match self {
            Self::Directory(dir) => dir.path(),
            Self::File(file) => file.path(),
        }
    }
}

#[derive(Debug)]
pub struct DirTree {
    root: Node,
}

impl DirTree {
    pub fn new(root: &Path) -> Result<Self, Error> {
        Ok(Self {
            root: Self::build_dir_tree(root)?,
        })
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn root_path(&self) -> &Path {
        self.root.path()
    }

    fn build_dir_tree(path: &Path) -> Result<Node, Error> {
        if path.is_dir() {
            let mut children = Vec::new();

            for child in fs::read_dir(path)? {
                let child = child?;
                let child_path = child.path();
                let child_node = Self::build_dir_tree(&child_path)?;
                children.push(child_node);
            }

            children.shrink_to_fit();

            Ok(Node::Directory(Directory {
                path: path.to_path_buf(),
                children,
            }))
        } else {
            Ok(Node::File(File {
                path: path.to_path_buf(),
            }))
        }
    }
}
