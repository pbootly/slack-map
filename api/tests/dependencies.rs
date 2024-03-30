use slackmap::packages::{build_package_tree, PackageInfo, PackageMap};


fn create_package_info(
    name: &str,
    version: &str,
    requires: &[&str],
) -> PackageInfo {
    PackageInfo {
        name: Some(name.to_string()),
        version: Some(version.to_string()),
        requires: requires.iter().map(|s| s.to_string()).collect(),
    }
}

#[test]
fn tree_creation() {
    let mut packages = PackageMap::new();
    let d1_dependencies = &["dmenu", 
    "libev", 
    "xcb-util-xrm", 
    "yajl", 
    "perl-AnyEvent", 
    "perl-JSON-XS"];
    let d2_dependencies = &["perl-Canary-Stability", "perl-Types-Serialiser"];
    let main_package = create_package_info("i3", "1", d1_dependencies);
    let package_with_dependencies = create_package_info("perl-JSON-XS", "4", d2_dependencies);
    for &dep in d1_dependencies.iter() {
        let p = create_package_info(&dep, "0", &[]);
        packages.insert(dep.to_string(), p);
    }
    packages.insert("i3".to_string(), main_package);
    packages.insert("perl-JSON-XS".to_string(), package_with_dependencies);
    let tree = build_package_tree(&packages, "i3");
    println!("{:?}", tree);
}
