pub struct Solution;
impl Solution {
    pub fn reformat_number(number: String) -> String {
        let mut result = String::new();
        let temp = number.replace(" ", "").replace("-", "");
        let number: Vec<String> = temp.chars().map(|x| x.to_string()).collect();
        let len = number.len();

        println!("{:?}", number);

        let mut i = 0;
        while i < len {
            if (len - i) == 4 {
                result += number[i].as_str();
                result += number[i + 1].as_str();
                result += "-";
                result += number[i + 2].as_str();
                result += number[i + 3].as_str();
                break;
            } else if len - i < 4 {
                for j in i..len {
                    result += number[j].as_str();
                }
                break;
            } else {
                result += number[i].as_str();
                result += number[i + 1].as_str();
                result += number[i + 2].as_str();
                result += "-";
                i += 3;
            }
        }

        result
    }
}
