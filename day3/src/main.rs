use input_reader::read_lines;
use std::collections::HashMap;
//use input_reader::scan;

fn main() {
    challenge1();
    challenge2();
}

fn challenge1() {
    let mut sum:u32 = 0;
    if let Ok(lines) = read_lines("input/input1.txt") {
        for line in lines { 
            let mut compartment1:String = line.expect("Unable to parse string");
            let compartment2:String = compartment1.split_off(compartment1.len()/2);
            for item in compartment1.chars() {
                if compartment2.contains(item) { sum += get_item_value(item); break; }
            }
        }
    }
    println!("Sum: {}", sum);
}

fn challenge2() {
    let mut sum:u32 = 0;
    let mut loop_num:u32 = 0;
    if let Ok(lines) = read_lines("input/input1.txt") {
        let mut group: [String;3] = Default::default();
        for line in lines { 
            group[(loop_num%3) as usize] = line.expect("Could not read line!");
            
            // Last element set
            if loop_num % 3 == 2 {
                let mut items: HashMap<char, usize> = HashMap::new();
                for i in 0..3 {
                    for c in group[i].chars() {
                        if *items.entry(c).or_insert(0) == i {
                            *items.get_mut(&c).expect("Failed to find key somehow") += 1;
                        }
                    }
                }
                for (k,v) in items { if v == 3 { sum += get_item_value(k); } }
            }

            loop_num+=1;
        }
    }
    println!("Sum: {}", sum);
}

fn get_item_value(item:char) -> u32 {
    if !item.is_ascii_alphabetic() {panic!("Unknown item char!")}
    if item.is_lowercase() {
        item as u32 - 97 + 1
    } else {
        item as u32 - 65 + 27
    }
}