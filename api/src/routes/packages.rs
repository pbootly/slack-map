use std::collections::HashMap;
use actix_web::{Responder,web};
use crate::packages::PackageInfo;

type Packages = web::Data<HashMap<String, PackageInfo>>;

pub async fn list_all_packages(packages: Packages) -> impl Responder {
    let hashmap = packages.get_ref().clone();

    web::Json(hashmap)
}
