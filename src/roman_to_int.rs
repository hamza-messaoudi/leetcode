/* 
Runtime: 31 ms, faster than 6.01% of Rust online submissions for Roman to Integer.
Memory Usage: 2.1 MB, less than 69.79% of Rust online submissions for Roman to Integer.

*/
#[allow(dead_code)]
fn roman_to_int(s: String) -> i32 {
    let mut number : i32 = 0;
    let mut minus : i32 = 0;
    for c in 0..s.split("").collect::<Vec<&str>>().len() {
        match s.split("").collect::<Vec<&str>>()[c] {

            "I" => match s.split("").collect::<Vec<&str>>()[c+1]{
                "V"=> {number+=4;minus+= 5;}
                "X"=> {number+=9;minus+= 10;}
                "L" |"C" |"D" |"M" |"I" |"" => number += 1,
                _=> {},
            },
            "V" => number += 5,
            "X" => match s.split("").collect::<Vec<&str>>()[c+1]{
                "L"=> {number+=40;minus+= 50;}
                "C"=> {number+=90;minus+= 100;}
                "I" |"V" |"D" |"M" |"X" |"" => number += 10,
                _=> {},
            },
            "L" => number += 50,
            "C" => match s.split("").collect::<Vec<&str>>()[c+1]{
                "D"=> {number+=400;minus+= 500;}
                "M"=> {number+=900;minus+= 1000;}
                "I" |"V" |"C" |"X" |"L" |"" => number += 100,
                _=> println!("ERROR"),
            },
            "D" => number += 500,
            "M" => number += 1000,

            _=> {},
        }
    }
    number - minus
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_game() {
        print!("{}", roman_to_int("IXII".to_string()))
    }
}
