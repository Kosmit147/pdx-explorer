use anyhow::{Context, Result, ensure};
use pdx_parse::ContentType;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirError {
    #[error("Root path `{0}` is not a directory")]
    TreeRootPathIsNotADirectory(PathBuf),
    #[error("Failed to extract file name from path `{0}`")]
    FailedToExtractFileName(PathBuf),
}

#[derive(Debug)]
pub struct Directory {
    full_path: PathBuf,
    relative_path: PathBuf,
    dir_name: PathBuf,

    content_type: ContentType,
    id: u32,

    children: Box<[Node]>,
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
pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new(root: &Path) -> Result<Self> {
        ensure!(
            root.is_dir(),
            DirError::TreeRootPathIsNotADirectory(root.to_path_buf())
        );

        TreeBuilder::new(root).build().with_context(|| {
            format!(
                "While building a directory tree with root path `{}`",
                root.display()
            )
        })
    }

    pub fn root(&self) -> &Node {
        &self.root
    }

    pub fn root_path(&self) -> &Path {
        self.root.full_path()
    }
}

// This is meant to be a private builder for use by this module only.
struct TreeBuilder<'a> {
    root_path: &'a Path,
    dir_ids: std::ops::RangeFrom<u32>,
    file_ids: std::ops::RangeFrom<u32>,
}

impl<'a> TreeBuilder<'a> {
    fn new(root: &'a Path) -> Self {
        Self {
            root_path: root,
            dir_ids: 0..,
            file_ids: 0..,
        }
    }

    fn build(mut self) -> Result<Tree> {
        Ok(Tree {
            root: self.create_node(self.root_path.to_owned())?,
        })
    }

    fn create_node(&mut self, path: PathBuf) -> Result<Node> {
        let relative_path = path.strip_prefix(self.root_path)?.to_path_buf();
        let file_name = PathBuf::from(
            path.file_name()
                .ok_or_else(|| DirError::FailedToExtractFileName(path.to_path_buf()))?,
        );
        let content_type = Self::get_content_type_from_relative_path(&relative_path);

        if path.is_dir() {
            let id = self.dir_ids.next().unwrap();
            let children = self.create_children_nodes(&path)?;

            Ok(Node::Directory(Directory {
                full_path: path,
                relative_path,
                dir_name: file_name,

                content_type,
                id,

                children,
            }))
        } else {
            let id = self.file_ids.next().unwrap();

            Ok(Node::File(File {
                full_path: path,
                relative_path,
                file_name,

                content_type,
                id,
            }))
        }
    }

    fn create_children_nodes(&mut self, path: &Path) -> Result<Box<[Node]>> {
        let mut children = Vec::new();

        for child in fs::read_dir(path)? {
            let child = child?;
            let child_path = child.path();
            let child_node = self.create_node(child_path)?;
            children.push(child_node);
        }

        Ok(children.into_boxed_slice())
    }

    // The given path must be relative to the root of the dir tree.
    fn get_content_type_from_relative_path(relative_path: &Path) -> ContentType {
        if relative_path.starts_with("localization") {
            ContentType::Localization
        } else {
            ContentType::Indeterminate
        }
    }
}
