use std::path::PathBuf;

pub fn lib_name_to_path(name: String) -> String {
    let parts: Vec<&str> = name.split(':').collect();
    let artifact_id = parts[1];
    let version = parts[2];
    let classifier = parts[3];
    let filename = format!("{}-{}-{}.jar", artifact_id, version, classifier);
    let mut path = PathBuf::new();
    path.push(parts[0]);
    path.push(artifact_id);
    path.push(version);
    path.push(filename);
    path.to_string_lossy().into_owned()
}