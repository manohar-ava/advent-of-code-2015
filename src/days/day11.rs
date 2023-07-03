pub fn solution() {
    let input: String = String::from("cqjxjnds");
    println!("DAY - 4");
    let first_ans = validate(input);
    println!("first ans = {first_ans}");
    let sec_ans = validate(first_ans);
    println!("second ans = {sec_ans}")
        
}

fn validate(string: String)->String {
    let mut input = string;
    let mut is_valid = false;
    
    while !is_valid {
        input = increment_once(&input);
        is_valid = is_stirng_valid(&input);
        // println!("{:?}",input);
    }
    return input
}

fn increment_once(string: &String) -> String {
    let mut ascii_char_vals: Vec<u8> = string.chars().map(|x| x as u8).collect();
    let mut updated_last =  false;
    let mut index = ascii_char_vals.len() - 1;
    let mut last_val = ascii_char_vals[index];
    let end_val = 122;
    let start_val = 97;

    while last_val == end_val {
        ascii_char_vals[index] = start_val;
        if index == 0 {
            updated_last = true;
            break;
        } else {
            index -= 1;
            last_val = ascii_char_vals[index];
        };
    }

    if !updated_last {
        ascii_char_vals[index] += 1;
    }
    let str:Vec<String> = ascii_char_vals.into_iter().map( |x|  String::from( x as char )).collect();
    str.join("")
}


fn is_stirng_valid(string:&String) ->bool {
    let includes_i_o_l = string.contains("i") || string.contains("o") || string.contains("l");
    let (has_first_pair , first_pair) = has_two_same_letters_together(&string , 0);
    let (has_sec_pair,_) = has_two_same_letters_together(&string, first_pair);
    return !includes_i_o_l && has_three_consicutive_letters(&string) && has_first_pair && has_sec_pair;
}

fn has_three_consicutive_letters(string: &String)->bool{ 
    let ascii_char_vals: Vec<u8> = string.chars().map(|x| x as u8).collect();
    let mut is_valid = false;
    for i in 1..ascii_char_vals.len()-1 {
        let cur = ascii_char_vals[i];
        if ascii_char_vals[i-1] == cur-1 && ascii_char_vals[i+1] == cur+1 {
            is_valid = true;
        }
    }
    is_valid
}

fn has_two_same_letters_together(string: &String, ex:u8)->(bool,u8){
    let ascii_char_vals: Vec<u8> = string.chars().map(|x| x as u8).collect();
    let mut is_valid = false;
    let mut val = 0;
    for i in 0..ascii_char_vals.len()-1 {
        let cur = ascii_char_vals[i];
        if ascii_char_vals[i+1] == cur  && cur != ex {
            is_valid = true;
            val = cur;
        }
    }
    (is_valid,val)
}