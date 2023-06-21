pub fn solution() {
    let g: String = String::from("3113322113");
    let mut input: Vec<char> = g.chars().collect();
    fn do_stuff(s: Vec<char>) -> Vec<char> {
        let mut rem: Vec<char> = vec![];
        let mut count = 1;
        for i in 0..s.len() {
            if i < s.len() - 1 && s[i] == s[i + 1] {
                count = count + 1
            } else {
                rem.push(char::from_digit(count as u32, 10).unwrap());
                rem.push(s[i]);
                count = 1;
            }
        }
        return rem;
    }
    let mut first: usize = 0;
    let second: usize;
    for i in 0..50 {
        input = do_stuff(input);
        if i == 39 {
            first = input.len()
        }
    }
    second = input.len();
    println!("DAY - 10");
    println!("first = {:?} second = {:?}", first, second);
}
