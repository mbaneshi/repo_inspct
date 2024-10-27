use git2::Repository;
// use std::fs;
//
// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let repo_path = ".";
//     let repo = Repository::open(repo_path)?;
//
//     let mut entries = repo.tree()?.iter();
//     while let Some(entry) = entries.next() {
//         let path = entry?.name().unwrap();
//         let obj = repo.find_object(entry?.id(), None)?;
//         if obj.kind() == Some(git2::ObjectType::Tree) {
//             println!("Directory: {}", path);
//         } else {
//             println!("File: {}", path);
//         }
//     }
//
//     Ok(())
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "ton"; // Replace with your desired path

    // Read the entries in the specified directory
    let entries = fs::read_dir(path)?;

    // Iterate over the entries
    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;

        // Check if the entry is a directory
        if metadata.is_dir() {
            println!("Directory: {}", entry.file_name().to_string_lossy());
        }
    }

    Ok(())
}
