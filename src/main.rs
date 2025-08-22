use std::fs;

fn main() {
    let path = "";
    // read lines from the file
    let content = fs::read_to_string(path).expect("Failed to read file");
    let lines = content.lines();
    for line in lines {
        println!("{}", line);
    }
}
