use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_from_file_commands(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let code: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect();

    code
}


fn main() {
    let data = load_from_file_commands("src/test.txt");
    let picks = &data[0];
    println!("Picks: {}", picks);
    
    // Calculate the total number of boards we are dealing with
    let total_boards = (&data.len()-1)/6;
    println!("\nBoards: {}", total_boards);

    // let boards: Vec<_> = vec![];
    
    for i in (2..data.len()).step_by(6) {
        for j in 0..5 {
            
            println!("line {}: {}", i+j, &data[i+j]);
        }
        println!("\n");

        // let mut board = Board {
        //     cells: Vec::new()
        // };
        // for j in 0..6 {
        //     let cell = Cell {
        //         value: data[i + j].parse::<i32>().unwrap(),
        //         marked: false,
        //     };
        //     board.cells.push(cell);
        // }
        // boards.push(board);
    }
}
