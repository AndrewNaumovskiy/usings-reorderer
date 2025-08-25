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

    // read lines from the file
    let content = fs::read_to_string(path).expect("Failed to read file");
    let lines = content.lines();

    let mut using_lines: Vec<String> = Vec::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        if line.contains("using ") {
            using_lines.push(line.replace('\u{feff}', ""));
        }
    }

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

    // sort grouped_usings by the minimum length of their values
    let mut sorted_grouped_usings: Vec<_> = grouped_usings.into_iter().collect();
    sorted_grouped_usings
        .sort_by_key(|(_, lines)| lines.iter().map(|line| line.len()).min().unwrap_or(0));

    let mut final_sorted_usings: Vec<String> = vec![];

    for (_, mut lines) in sorted_grouped_usings {
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
    let mut new_content = content_lines.join("\n");
    new_content.push('\n');
    fs::write(path, new_content).expect("Failed to write file");
}
