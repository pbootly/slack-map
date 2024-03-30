use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub type PackageMap = HashMap<String, PackageInfo>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: Option<String>,
    pub version: Option<String>,
    pub requires: Vec<String>,
}

impl Default for PackageInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl PackageInfo {
    pub fn new() -> Self {
        PackageInfo {
            name: None,
            version: None,
            requires: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    name: String,
    children: Vec<Node>,
}

impl Node {
    pub fn new(name: String, children: Vec<Node>) -> Node {
        Node {
            name,
            children,
        }
    }
}

pub fn build_package_tree(packages: &PackageMap, root_package: &str) -> Node {
    let mut tree = Node::new(root_package.to_string(), Vec::new());
    let mut children = Vec::new();
    if let Some(package) = packages.get(root_package) {
        for dependency in &package.requires {
            let child_tree = build_package_tree(packages, dependency);
            children.push(child_tree);
        }
    } else {
        println!("Package not found: {:?}", root_package);
    }
    tree.children = children;

    tree
}
