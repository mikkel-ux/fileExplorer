use serde::Serialize;
use strsim::levenshtein;
use tauri::fs::Scope;

#[derive(Serialize)]
pub struct File {
    name: String,
}

#[tauri::command]
pub fn search_files(query: String) -> Vec<File> {
    let foo_data = vec![
        File {
            name: "home/foo.txt".to_string(),
        },
        File {
            name: "home/bar.txt".to_string(),
        },
        File {
            name: "home/baz.txt".to_string(),
        },
        File {
            name: "home/qux.txt".to_string(),
        },
        File {
            name: "home/quux.txt".to_string(),
        },
        File {
            name: "home/corge.txt".to_string(),
        },
        File {
            name: "home/PATH.txt".to_string(),
        },
    ];
    let new_query = query.split("/").last().unwrap().to_lowercase();

    let mut filtered_files: Vec<(File, i32)> = foo_data
        .into_iter()
        .map(|file| {
            let filename = file.name.split('/').last().unwrap().to_lowercase();
            let score = calculate_score(&filename, &new_query);
            (file, score)
        })
        .filter(|(_, score)| *score > 0)
        .collect();

    filtered_files.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.name.cmp(&b.0.name)));
    filtered_files.into_iter().map(|(file, _)| file).collect()
}

fn calculate_score(filename: &str, query: &str) -> i32 {
    if filename == query {
        return 100;
    } else if filename.starts_with(query) {
        return 90;
    } else if filename.contains(query) {
        return 80;
    }

    let distance = levenshtein(filename, &query);
    let distance_score = 30 - distance as i32 * 5;
    distance_score.max(0)
}
