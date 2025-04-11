use crate::files::get_dirs;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use serde::Deserialize;
use serde::Serialize;
use strsim::levenshtein;

#[derive(Serialize, Deserialize)]
pub struct File {
    name: String,
}

/* let foo_data = vec![
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
]; */
#[tauri::command]
pub fn search_files(path: String) -> Vec<String> {
    //let query = path.split('/').last().unwrap_or("").to_lowercase();
    println!("Path: {}", path);
    let mut parts = path.rsplitn(2, "/");
    println!("Parts: {:?}", parts.clone().collect::<Vec<_>>());
    let query = parts.next().unwrap_or("").to_lowercase();
    let base = parts.next().unwrap_or("").to_lowercase();
    let matcher = SkimMatcherV2::default();
    let dirs = match get_dirs(base.clone()) {
        Ok(dirs) => dirs,
        Err(_) => return vec![],
    };
    println!("Query: {}", query);
    println!("Base: {}", base);
    println!("Dirs: {:?}", dirs);

    let mut filtered_files: Vec<(String, i32)> = dirs
        .into_iter()
        .filter_map(|file_path| {
            let filename = file_path.split('/').last()?.to_lowercase();
            let score = fuzzy_score(&filename, &query, &matcher);
            if score > 0 {
                Some((file_path, score))
            } else {
                None
            }
        })
        .collect();

    filtered_files.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    filtered_files.into_iter().map(|(file, _)| file).collect()
}

fn fuzzy_score(filename: &str, query: &str, matcher: &SkimMatcherV2) -> i32 {
    if filename == query {
        return 100;
    } else if filename.starts_with(query) {
        return 90;
    } else if filename.contains(query) {
        return 80;
    }

    let distance = levenshtein(filename, &query);
    let distance_score = 30 - distance as i32 * 5;

    let fuzzy_score = matcher.fuzzy_match(filename, query).unwrap_or(0) as i32;
    (distance_score + (fuzzy_score / 5)).max(0)
}
