use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mytool <file>");
        return;
    }

    let path = &args[1];

    //let path = "C:\\AutoloadItRepos\\Desktop\\AutoLoadIT\\AutoLoadIT\\ViewModels\\Tabs\\DocumentTabViewModel.cs";

    // read lines from the file
    let content = fs::read_to_string(path).expect("Failed to read file");
    let lines = content.lines();

    // find index of first line without using
    let first_non_using_line_index = lines.clone().position(|line| !line.contains("using "));

    // get first 5 lines
    let using_lines: Vec<&str> = lines
        .clone()
        .take(first_non_using_line_index.unwrap_or(0))
        .collect();

    // group usings by first word in split by '.' ("System.IO")
    let mut grouped_usings: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in using_lines {
        let namespace = line.split_whitespace().nth(1).unwrap_or("");
        if namespace.contains('.') {
            let first_word = namespace.split('.').next().unwrap_or("");

            grouped_usings
                .entry(first_word)
                .or_insert_with(Vec::new)
                .push(line);
        } else {
            let cleaned_namespace = namespace.split(";").next().unwrap_or("");
            grouped_usings
                .entry(cleaned_namespace)
                .or_insert_with(Vec::new)
                .push(line);
        }
    }

    // sort by grouped_usings by key length
    let mut sorted_grouped_usings: Vec<_> = grouped_usings.into_iter().collect();
    sorted_grouped_usings.sort_by_key(|(key, _)| key.len());

    let mut final_sorted_usings = vec![];

    for (_first_word, mut lines) in sorted_grouped_usings {
        // sort lines by length
        lines.sort_by_key(|line| line.len());
        for line in lines {
            final_sorted_usings.push(line);
        }
    }

    // overwrite path first lines with final_sorted_usings
    let mut content_lines = content.lines().collect::<Vec<_>>();
    content_lines.splice(0..final_sorted_usings.len(), final_sorted_usings);
    let new_content = content_lines.join("\n");
    fs::write(path, new_content).expect("Failed to write file");
}
