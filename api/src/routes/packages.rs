use crate::packages::{build_package_tree, Node, PackageInfo};
use actix_web::{web, Responder};
use std::collections::HashMap;

type Packages = web::Data<HashMap<String, PackageInfo>>;

pub async fn list_all_packages(packages: Packages) -> impl Responder {
    let hashmap = packages.get_ref().clone();

    web::Json(hashmap)
}

pub async fn get_package_tree(
    packages: Packages,
    mut query: web::Query<HashMap<String, String>>,
) -> impl Responder {
    let hashmap = packages.get_ref().clone();
    if let Some(package_name) = query.get_mut("name") {
        match hashmap.get(package_name) {
            Some(_) => {
                let tree = build_package_tree(&hashmap, package_name);
                return web::Json(tree);
            }
            None => {
                let not_found_node = Node::new("404 Not Found".to_string(), vec![]);
                return web::Json(not_found_node);
            }
        }
    } else {
        let not_found_node = Node::new("404 Not Found".to_string(), vec![]);
        return web::Json(not_found_node);
    }
}
