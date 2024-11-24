use std::{
    fs::File,
    io::{self, BufRead},
};

fn read_input() -> Vec<String> {
    let mut lines = Vec::new();

    let file_path = "quest1_note.txt";

    if let Ok(file) = File::open(&file_path) {
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            if let Ok(content) = line {
                lines.push(content);
            }
        }
    } else {
        eprintln!("Error: the file {} does not exist: ", file_path);
    }

    lines
}

pub fn count_and_sum_characters() -> i64 {
    let lines = read_input();

    let mut total_potions = 0;

    for line in lines {
        for c in line.chars() {
            match c {
                'A' => total_potions += 0,
                'B' => total_potions += 1,
                'C' => total_potions += 3,
                _ => {}
            }
        }
    }

    total_potions
}

#[cfg(test)]
mod tests {
    use io::Write;

    use super::*;
    use std::fs;

    fn create_test_file(content: &str) -> std::io::Result<()> {
        let mut file = File::create("quest1_note.txt")?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn clean_up_test_file() {
        let _ = fs::remove_file("quest1_note.txt");
    }

    #[test]
    fn test_example_sequence() {
        create_test_file("ABBAC").unwrap();
        assert_eq!(count_and_sum_characters(), 5);
        clean_up_test_file();
    }
}
