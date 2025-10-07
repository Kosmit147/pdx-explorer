use crate::core::*;
use crate::database::ContentType;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Node {
    Directory {
        path: PathBuf,
        content_type: ContentType,
        children: Vec<Node>,
        id: u32,
    },
    File {
        path: PathBuf,
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
        if !root.is_dir() {
            fail!(
                "Root path `{}` doesn't point to a directory",
                root.display()
            );
        }

        Ok(Self {
            root: Self::create_node(root, root, &mut (0..), &mut (0..))?,
        })
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn root_path(&self) -> &Path {
        self.root.path()
    }

    fn create_node(
        root_path: &Path,
        current_path: &Path,
        dir_ids: &mut std::ops::RangeFrom<u32>,
        file_ids: &mut std::ops::RangeFrom<u32>,
    ) -> Result<Node> {
        let path = current_path.to_path_buf();
        let content_type = Self::get_content_type_from_path(root_path, current_path)?;

        if current_path.is_dir() {
            let id = dir_ids.next().unwrap();
            let children = Self::create_children_nodes(root_path, current_path, dir_ids, file_ids)?;

            Ok(Node::Directory {
                path,
                content_type,
                children,
                id,
            })
        } else {
            let id = file_ids.next().unwrap();

            Ok(Node::File {
                path,
                content_type,
                id,
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
    fn get_content_type_from_path(root_path: &Path, path: &Path) -> Result<ContentType> {
        let relative_path = path.strip_prefix(root_path)?;

        Ok(if relative_path.starts_with("localization") {
            ContentType::Localization
        } else {
            ContentType::Indeterminate
        })
    }
}
