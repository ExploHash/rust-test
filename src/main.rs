use std::string;

const MAJOR_INTERVALS: [i8; 7] = [2, 2, 1, 2, 2, 2, 1];
const MINOR_INTERVALS: [i8; 7] = [2, 1, 2, 2, 1, 2, 2];
const NOTES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

fn main() {
    println!("Scale pitches finder v0.1");

    //Grab input
    let root_note = ask_for_input("Enter root note: ");
    let scale_type = ask_for_input("Enter scale type (major, minor): ");

    //Find scale pitches
    let pitches = calculate_pitches(root_note, scale_type);

    //Print results
    for pitch in pitches {
        print!("{} ", pitch);
    }

}

fn ask_for_input(question: &str) -> String {
    let mut input = String::new();
    println!("{}", question);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn calculate_pitches(root_note: String, scale_type: String) -> Vec<String> {
    let mut pitches: Vec<String> = Vec::new();

    let root_note_index = NOTES.iter().position(|&n| n == root_note).unwrap();
    let intervals: [i8; 7];
    if scale_type == "major" {
        intervals = MAJOR_INTERVALS;
    } else {
        intervals = MINOR_INTERVALS;
    }
    
    let mut diff: i8 = 0;
    for note_counter in 0..7 {
        diff += intervals[note_counter];
        let note_index: usize = (root_note_index + diff as usize) % NOTES.len();
        pitches.push(NOTES[note_index as usize].to_string());
    }
    pitches
}
