use std::cmp::Reverse;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug)]
struct ElfInventory {
    contents: Vec<i32>,
    sum: i32,
}

fn main() {
    let mut elves_inventory: Vec<ElfInventory> = Vec::new();
    let mut elves_inventory_sum: Vec<i32> = Vec::new();
    // File hosts must exist in current path before this produces output
    match read_lines("input") {
        Ok(lines) => {
            // Consumes the iterator, returns an (Optional) String
            let mut index: usize = 0;
            for (lines_counter, line) in lines.flatten().enumerate() {
                if lines_counter == 0 && line.is_empty() {
                    panic!("First line in input file must not be empty!");
                }

                if line.is_empty() {
                    let sum: i32 = elves_inventory[index].contents.clone().into_iter().sum();
                    elves_inventory[index].sum = sum;
                    elves_inventory_sum.push(sum);

                    index += 1;
                    continue;
                } else {
                    if elves_inventory.len() <= index {
                        elves_inventory.push(ElfInventory {
                            contents: Vec::new(),
                            sum: 0,
                        });
                    }
                    elves_inventory[index]
                        .contents
                        .push(line.parse::<i32>().unwrap());
                }
            }
            elves_inventory_sum.sort_by_key(|w| Reverse(*w));

            println!(
                "Elves's largest calories inventory: {}",
                elves_inventory_sum[0]
            );

            println!(
                "Elves's top 3 calories inventories sum: {}",
                elves_inventory_sum[0] + elves_inventory_sum[1] + elves_inventory_sum[2]
            );
        }
        Err(e) => println!("Failed to read input data: {}", e),
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
