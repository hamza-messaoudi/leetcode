/* 

Runtime: 4 ms, faster than 65.17% of Rust online submissions for Roman to Integer.
Memory Usage: 2.1 MB, less than 36.78% of Rust online submissions for Roman to Integer.

*/
#[allow(dead_code)]
fn roman_to_int(s: String) -> i32 {
    let mut number : i32 = 0;
    let mut previous_number : &str ="";
    let iter = s.split("");

    for i in iter {
        match i {
            
            "I" => number +=1,
            
            "V" => {match previous_number {
                "I" => number+= 3,
                _=> number+= 5,
            }},

            "X" => {match previous_number {
               "I" => number+= 8,
               _=> number+= 10,
            }},
            
            "L" => {match previous_number {
                "X" => number+= 30,
                _=> number+= 50,
            }},
            
            "C" => {match previous_number {
                "X" => number+= 80,
                _=> number+= 100,
            }},
            
            "D" => {match previous_number {
                "C" => number+= 300,
                _=> number+= 500,
            }},

            "M" => {match previous_number {
                "C" => number+= 800,
                _=> number+= 1000,
            }},
            
            

            _=> {},
        }
        previous_number = i;
    }
    number 
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_game() {
        print!("{}", roman_to_int("IXII".to_string()))
    }
}
