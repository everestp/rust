use std::fs::read_to_string;

fn main() {
    let file_path = String::from("a.txt");
    match read_from_file_everest(file_path) {
        Ok(contents) => println!("File contents:\n{}", contents),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}

fn read_from_file_everest(file_path: String) -> Result<String, String> {
    match read_to_string(file_path) {
        Ok(data) => Ok(data),
        Err(err) => Err(format!("File not read: {}", err)),
    }
}
