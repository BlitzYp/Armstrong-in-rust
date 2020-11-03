fn main() {
    let num = 371;
    let res = armstrong(num);
    println!("{}", res)
}

fn armstrong<'a>(num: i32) -> &'a str {
    let mut sum: i32 = 0;
    let count: usize = num.to_string().chars().count();
    let mut play = num;
    while play != 0 {
        let num_to_power = play % 10;
        sum += i32::pow(num_to_power, count as u32);
        play /= 10;
    }

    if sum == num {
        "armstrong"
    } else {
        "not armstrong"
    }
}
