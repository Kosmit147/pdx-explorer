use crate::core::*;
use std::path::{Path, PathBuf};
use std::{fmt, fs};

#[derive(Debug, Clone, Copy)]
pub enum ContentType {
    Localization,
    Other,
}

impl Default for ContentType {
    fn default() -> Self {
        Self::Other
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Localization => "Localization",
                Self::Other => "Other",
            }
        )
    }
}

#[derive(Debug)]
pub enum Node {
    Directory {
        path: PathBuf, // This path is relative to the root path.
        content_type: ContentType,
        children: Vec<Node>,
        id: u32,
    },
    File {
        path: PathBuf, // This path is relative to the root path.
        content_type: ContentType,
        id: u32,
    },
}

impl Node {
    pub fn path(&self) -> &Path {
        match self {
            Self::Directory { path, .. } => path,
            Self::File { path, .. } => path,
        }
    }

    pub fn content_type(&self) -> ContentType {
        match self {
            Self::Directory { content_type, .. } => *content_type,
            Self::File { content_type, .. } => *content_type,
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            Self::Directory { id, .. } => *id,
            Self::File { id, .. } => *id,
        }
    }
}

#[derive(Debug)]
pub struct DirTree {
    root: Node,
}

impl DirTree {
    pub fn new(root: &Path) -> Result<Self> {
        Ok(Self {
            root: Self::create_root_node(root)?,
        })
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn root_path(&self) -> &Path {
        self.root.path()
    }

    fn create_root_node(path: &Path) -> Result<Node> {
        if !path.is_dir() {
            return Err(error!(
                "Root path `{}` doesn't point to a directory.",
                path.display()
            ));
        }

        let mut dir_ids = 1..; // 0 is reserved for the root node.
        let mut file_ids = 0..;

        let children = Self::create_children_nodes(path, path, &mut dir_ids, &mut file_ids)?;

        Ok(Node::Directory {
            path: path.to_path_buf(),
            content_type: Default::default(),
            children,
            id: 0,
        })
    }

    fn create_node(
        root_path: &Path,
        current_path: &Path,
        dir_ids: &mut std::ops::RangeFrom<u32>,
        file_ids: &mut std::ops::RangeFrom<u32>,
    ) -> Result<Node> {
        let relative_path = current_path.strip_prefix(root_path)?;
        let content_type = Self::get_content_type_from_path(relative_path);

        if current_path.is_dir() {
            let children = Self::create_children_nodes(root_path, current_path, dir_ids, file_ids)?;

            Ok(Node::Directory {
                path: relative_path.to_path_buf(),
                content_type,
                children,
                id: dir_ids.next().unwrap(),
            })
        } else {
            Ok(Node::File {
                path: relative_path.to_path_buf(),
                content_type,
                id: file_ids.next().unwrap(),
            })
        }
    }

    fn create_children_nodes(
        root_path: &Path,
        path: &Path,
        dir_ids: &mut std::ops::RangeFrom<u32>,
        file_ids: &mut std::ops::RangeFrom<u32>,
    ) -> Result<Vec<Node>> {
        let mut children = Vec::new();

        for child in fs::read_dir(path)? {
            let child = child?;
            let child_path = child.path();
            let child_node = Self::create_node(root_path, &child_path, dir_ids, file_ids)?;
            children.push(child_node);
        }

        children.shrink_to_fit();
        Ok(children)
    }

    // The given path must be relative to the root of the dir tree.
    fn get_content_type_from_path(path: &Path) -> ContentType {
        if path.starts_with("localization") {
            ContentType::Localization
        } else {
            ContentType::Other
        }
    }
}
