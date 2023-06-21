use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

pub fn solution (){
let file_contents = fs::read_to_string("9.txt").expect("Should have been able to read the file");

let lines  = file_contents.lines();
let mut countries: HashSet<&str> =  HashSet::new();
let mut path_values: HashMap<String, i32> = HashMap::new();
let mut all_paths:Vec<i32> = vec![];
for line in lines {
    let str_vec:Vec<&str> = line.split(" ").collect();
    let from = str_vec.get(0).unwrap();
    let to = str_vec.get(2).unwrap();
    countries.insert(*from);
    countries.insert(*to);

    let key1 =  [*from,*to].join("-");
    let key2=  [*to,*from].join("-");
    let val = str_vec.get(4).unwrap().to_string();
    let parsed_val = val.parse::<i32>().unwrap();
    path_values.insert(key1, parsed_val);
    path_values.insert(key2, parsed_val);
}

fn rec(arr: &mut Vec<&str>,index:usize,weights:&HashMap<String, i32>,pts: &mut Vec<i32>) {
    if index == arr.len(){
        let mut count = 0;
        for i in 0..arr.len()-1 {
            let s = [arr[i],arr[i+1]].join("-");
            match weights.get(&s) {
                Some(n) => {
                    count += *n
                },
                None=>{}
            }
        }
        pts.push(count);
        return;
    }

    for j in index..arr.len(){
        arr.swap(index, j);
        rec(arr, index+1, weights, pts);
        arr.swap(j, index)
    }
    return;
}

let mut c:Vec<&str> = countries.into_iter().collect();
rec(&mut c, 0, &path_values, &mut all_paths);

println!("DAY - 9");
println!("first answer - {:?}\nsecond answer - {:?}",all_paths.iter().min(),all_paths.iter().max())
}
