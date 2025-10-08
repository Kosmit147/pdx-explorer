use crate::core::*;
use crate::database::ContentType;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Directory {
    full_path: PathBuf,
    relative_path: PathBuf,
    dir_name: PathBuf,

    content_type: ContentType,
    id: u32,

    children: Vec<Node>,
}

impl Directory {
    pub fn full_path(&self) -> &Path {
        &self.full_path
    }

    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub fn dir_name(&self) -> &Path {
        &self.dir_name
    }

    pub fn content_type(&self) -> ContentType {
        self.content_type
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn children(&self) -> &[Node] {
        &self.children
    }
}

#[derive(Debug)]
pub struct File {
    full_path: PathBuf,
    relative_path: PathBuf,
    file_name: PathBuf,

    content_type: ContentType,
    id: u32,
}

impl File {
    pub fn full_path(&self) -> &Path {
        &self.full_path
    }

    pub fn relative_path(&self) -> &Path {
        &self.relative_path
    }

    pub fn file_name(&self) -> &Path {
        &self.file_name
    }

    pub fn content_type(&self) -> ContentType {
        self.content_type
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
pub enum Node {
    Directory(Directory),
    File(File),
}

impl Node {
    pub fn full_path(&self) -> &Path {
        match self {
            Self::Directory(dir) => dir.full_path(),
            Self::File(file) => file.full_path(),
        }
    }

    pub fn relative_path(&self) -> &Path {
        match self {
            Self::Directory(dir) => dir.relative_path(),
            Self::File(file) => file.relative_path(),
        }
    }

    pub fn file_name(&self) -> &Path {
        match self {
            Self::Directory(dir) => dir.dir_name(),
            Self::File(file) => file.file_name(),
        }
    }

    pub fn content_type(&self) -> ContentType {
        match self {
            Self::Directory(dir) => dir.content_type(),
            Self::File(file) => file.content_type(),
        }
    }

    pub fn id(&self) -> u32 {
        match self {
            Self::Directory(dir) => dir.id(),
            Self::File(file) => file.id(),
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
        self.root.full_path()
    }

    fn create_node(
        root_path: &Path,
        current_path: &Path,
        dir_ids: &mut std::ops::RangeFrom<u32>,
        file_ids: &mut std::ops::RangeFrom<u32>,
    ) -> Result<Node> {
        let full_path = current_path.to_path_buf();
        let relative_path = full_path.strip_prefix(root_path)?.to_path_buf();
        let file_name = PathBuf::from(full_path.file_name().ok_or_else(|| {
            error!(
                "Failed to extract file name from path `{}`",
                full_path.display()
            )
        })?);
        let content_type = Self::get_content_type_from_relative_path(&relative_path)?;

        if current_path.is_dir() {
            let id = dir_ids.next().unwrap();
            let children = Self::create_children_nodes(root_path, current_path, dir_ids, file_ids)?;

            Ok(Node::Directory(Directory {
                full_path,
                relative_path,
                dir_name: file_name,

                content_type,
                id,

                children,
            }))
        } else {
            let id = file_ids.next().unwrap();

            Ok(Node::File(File {
                full_path,
                relative_path,
                file_name,

                content_type,
                id,
            }))
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
    fn get_content_type_from_relative_path(relative_path: &Path) -> Result<ContentType> {
        Ok(if relative_path.starts_with("localization") {
            ContentType::Localization
        } else {
            ContentType::Indeterminate
        })
    }
}
