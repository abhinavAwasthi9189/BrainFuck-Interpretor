use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    //we start with 300 not that many, as it is a vector size can be increased later.
    let mut base: Vec<u8> = vec![0; 300];

    let code: Vec<char> = strput().chars().collect();

    // Maps every '[' directly to its matching ']' and vice-versa
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut stack: Vec<usize> = Vec::new();
    
    //this may make the time complexity of the code O[2n] but it makes it much easier to code in.
    for (i, &ch) in code.iter().enumerate() {
        if ch == '[' {
            stack.push(i);
        } else if ch == ']' {
            let j = stack.pop().expect("illegal code no [ for the given ] at {i}");
            map.insert(j, i);
            map.insert(i, j);
        }
    }
    if !stack.is_empty() {
        panic!("unclosed delimeters found");
    }
    
    //these are the pointers pc for array and ptr for code.
    let mut pc: usize = 0;
    let mut ptr: usize = 0;

    while ptr < code.len() {
        let val = code[ptr];
        match val {
            '<' => { if pc > 0 { pc -= 1; } }
            '>' => { pc += 1; if pc >= base.len() { base.push(0); } }
            '+' => { base[pc] = base[pc].wrapping_add(1); }
            '-' => { base[pc] = base[pc].wrapping_sub(1); }
            ',' => { base[pc] = intput(); }
            '.' => { print_ascii(base[pc]); }
            '[' => {
                if base[pc] == 0 {
                    ptr = *map.get(&ptr).unwrap();
                }
            }
            ']' => {
                if base[pc] != 0 {
                    ptr = *map.get(&ptr).unwrap();
                }
            }
            _ => {} 
        }
        ptr += 1;
    }

    io::stdout().flush().unwrap();
    print!("\n");
}

fn strput() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn intput() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn print_ascii(value: u8) {
    print!("{}", value as char);
}
