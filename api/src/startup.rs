use std::collections::HashMap;
use std::net::TcpListener;
use walkdir::WalkDir;
use actix_web::{dev::Server, web, App, HttpServer};
use actix_cors::Cors;
use crate::parser::parse_file;
use crate::packages::{PackageInfo, PackageMap};
use crate::routes::{health_check, packages::*};

fn json_packages_from_files(
    path: &str,
    mut packages: PackageMap,
    ) -> PackageMap 
{
    for entry in WalkDir::new(path)
        .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            {
                let f_name = entry.file_name().to_string_lossy();
                if f_name.ends_with(".info") {
                    match parse_file(entry.path()) {
                        Ok(package_info) => {
                            if let Some(name) = &package_info.name {
                                packages.insert(name.clone(), package_info);
                            }
                        },
                        Err(e) => {
                            println!("Failed to parse file {}", e);
                        }
                    }
                }
            }
    packages
}

pub fn run(
    listener: TcpListener,
) -> Result<Server, std::io::Error> {
    let hashmap: HashMap<String, PackageInfo> = HashMap::new();
    let packages = json_packages_from_files(".", hashmap.clone());
    let shared = web::Data::new(packages);
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec!["Content-Type"]);

        App::new()
            .wrap(cors)
            .app_data(shared.clone())
            .route("/health_check", web::get().to(health_check))
            .route("/packages", web::get().to(list_all_packages))
            .route("/package", web::get().to(get_package_tree))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
