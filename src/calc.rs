use regex::Regex;
use meval;
use colored::Colorize;

#[derive(Debug)]
pub enum CalcError {
    InvalidExpression,
}

fn parse_expression(input: &str) -> Result<String, CalcError> {
    let re = Regex::new(r"\s*(0x[0-9A-Fa-f]+|0b[01]+|[0-9]+|[+\-*/()])\s*").unwrap();

    let mut expression = Vec::new();
    let mut matched_string = String::new();

    for cap in re.captures_iter(input) {
        let token = cap[1].trim().to_string();
        matched_string.push_str(&token);
        matched_string.push(' ');

        if token.starts_with("0x") {
            if let Ok(num) = u32::from_str_radix(&token[2..], 16) {
                expression.push(num.to_string());
            } else {
                return Err(CalcError::InvalidExpression);
            }

        } else if token.parse::<u32>().is_ok() {
            if let Ok(num) = u32::from_str_radix(&token, 10) {
                expression.push(num.to_string());
            } else {
                return Err(CalcError::InvalidExpression);
            }

        } else if token.starts_with("0b") {
            if let Ok(num) = u32::from_str_radix(&token[2..], 2) {
                expression.push(num.to_string());
            } else {
                return Err(CalcError::InvalidExpression);
            }

        } else if ["+", "-", "*", "/", "(", ")"].contains(&token.as_str()) {
            expression.push(token.to_string());

        } else {
            return Err(CalcError::InvalidExpression);
        }
    }

    // 正規表現に合致するものだけが存在するかどうかを確認する
    // 上のコードでは合致するものだけが認識されpushされる
    // pushされてできた文字列が元の入力と同じなら全て正規表現に合致する
    if input.trim().replace(" ", "") != matched_string.trim().replace(" ", "") {
        return Err(CalcError::InvalidExpression);
    }

    Ok(expression.join(" "))
}

pub fn display_number(num: u32) {
    let mut binary =format!("{:b}", num).to_string();
    let mut len = binary.len();
    let padlen = (8-(len%8))%8;
    let mut pad = String::new();
    for _ in 0..padlen {
        pad.push('0');
    }
    binary.insert_str(0, pad.as_str());

    len = binary.len();
    if binary.len() > 8 {
        for i in (8..len).step_by(8) {
            binary.insert(i, ' ');
        }
    }

    println!("[{}] Hex:\t0x{:X}", "calc".green(), num);
    println!("[{}] Dec:\t{}", "calc".green(), num);
    println!("[{}] Bin:\t{}", "calc".green(), binary);
}

pub fn calc(str: &str) -> Result<u32, CalcError> {
    let expression = match parse_expression(str) {
        Ok(expr) => expr,
        Err(_) => {
            return Err(CalcError::InvalidExpression);
        }
    };

    let number = match meval::eval_str(&expression) {
        Ok(num) => num as u32,
        Err(_) => {
            return Err(CalcError::InvalidExpression);
        }
    };

    Ok(number)
}
