use std::collections::HashMap;
use std::io;
pub fn add(v: &mut Vec<i32>) {
    let mut insert = String::new(); // hold the input
    loop {
        println!("Type a number or `done`: ");
        io::stdin().read_line(&mut insert).expect("failed"); // read from stdin
        if insert.trim().eq("done") {
            stats(v);
            break;
        }
        let num: i32 = match insert.trim().parse() {
            // convert string to int
            Ok(num) => num,
            Err(_) => -1,
        };
        insert.clear();
        v.push(num);
    }
}

fn stats(v: &Vec<i32>) {
    let mut map: HashMap<i32, i32> = HashMap::new();
    for i in v {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }
    println!("The median is: {}", v[v.len() / 2]);
    for (key, val) in map {
        println!("Num: {} Count: {}", key, val);
    }
    return;
}
