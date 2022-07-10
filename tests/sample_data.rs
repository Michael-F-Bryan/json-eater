use std::{collections::BTreeMap, path::Path};

#[test]
#[ignore = "Only run this when you want to update sample-data.txt"]
fn update_registry() {
    let benchmark_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("benches");
    let data_dir = benchmark_dir.join("data");
    let registry_path = benchmark_dir.join("sample-data.txt");
    let upstream_url = "https://www.hl7.org/fhir/";

    let _ = std::fs::remove_dir_all(&data_dir);
    std::fs::create_dir_all(&data_dir).unwrap();

    let files = ["patient-example.json", "patient-example-proband.json"];
    let mut registry = BTreeMap::new();

    for file in files {
        let url = format!("{upstream_url}/{file}");
        let dest = data_dir.join(file);
        let hash = fetch_data::hash_download(url, dest).unwrap();
        registry.insert(file, hash);
    }

    let mut updated = String::new();

    for (filename, hash) in registry {
        updated.push_str(filename);
        updated.push(' ');
        updated.push_str(&hash);
        updated.push('\n');
    }

    match std::fs::read_to_string(&registry_path) {
        Ok(previous) if previous == updated => {},
        _ => std::fs::write(&registry_path, updated).unwrap(),
    }
}
