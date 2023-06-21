pub fn solution() {
    let input: &str = "ckczppom";
    let mut i = 1;
    let mut first_ans = 0;
    let sec_ans;
    loop {
        let digest = md5::compute(format!("{input}{i}"));
        let formatted_str = format!("{digest:x}");
        if formatted_str.starts_with("00000") && first_ans == 0 {
            first_ans = i;
        }
        if formatted_str.starts_with("000000") {
            sec_ans = i;
            break;
        }
        i += 1;
    }
    println!("DAY - 4");
    println!(" first ans = {first_ans}  \n second ans = {sec_ans}")
}
