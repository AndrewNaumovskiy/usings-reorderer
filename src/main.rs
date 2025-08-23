use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: usings-reorderer <file>");
        return;
    }

    let path = &args[1];

    //let path = "C:\\AutoloadItRepos\\Desktop\\AutoLoadIT\\AutoLoadIT\\ViewModels\\Tabs\\DocumentTabViewModel.cs";

    // read lines from the file
    let content = fs::read_to_string(path).expect("Failed to read file");
    let lines = content.lines();

    let using_lines: Vec<String> = lines
        .filter(|line| line.contains("using "))
        .map(|line| line.replace('\u{feff}', ""))
        .collect();

    let mut grouped_usings: HashMap<String, Vec<String>> = HashMap::new();

    for using in using_lines {
        let namespace = using.split_whitespace().nth(1).unwrap_or("").to_string();

        let cleaned_namespace = if namespace.contains('.') {
            namespace.split('.').next().unwrap_or("").to_string()
        } else {
            namespace.split(';').next().unwrap_or("").to_string()
        };

        grouped_usings
            .entry(cleaned_namespace)
            .or_insert_with(Vec::new)
            .push(using);
    }

    // sort by grouped_usings by key length
    let mut sorted_grouped_usings: Vec<_> = grouped_usings.into_iter().collect();
    sorted_grouped_usings.sort_by_key(|(key, _)| key.len());

    let mut final_sorted_usings: Vec<String> = vec![];

    for (_first_word, mut lines) in sorted_grouped_usings {
        // sort lines by length
        lines.sort_by_key(|line| line.len());
        for line in lines {
            final_sorted_usings.push(line);
        }
    }

    // overwrite path first lines with final_sorted_usings
    let mut content_lines = content.lines().collect::<Vec<_>>();
    let final_sorted_usings_refs: Vec<&str> =
        final_sorted_usings.iter().map(|s| s.as_str()).collect();
    content_lines.splice(0..final_sorted_usings_refs.len(), final_sorted_usings_refs);
    let new_content = content_lines.join("\n");
    fs::write(path, new_content).expect("Failed to write file");
}
