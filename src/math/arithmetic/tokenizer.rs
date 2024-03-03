const VALID_TOKENS: [char; 15] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.', '+', '-', '*', '/',
];

pub fn tokenize(expression: &str) -> Vec<String> {
    let mut tokenized = Vec::new();

    let mut digit = String::new();
    let mut in_digit = false;

    for i in expression.chars() {
        if i.is_whitespace() {
            if in_digit {
                tokenized.push(digit.clone());

                digit = String::new();
                in_digit = false;
            }

            continue;
        }

        if !VALID_TOKENS.contains(&i) {
            panic!("Invalid token");
        }

        if i.is_ascii_digit() || i == '.' {
            if !in_digit {
                in_digit = true;
            }

            digit.push(i);
            continue;
        }

        if in_digit {
            tokenized.push(digit.clone());

            digit = String::new();
            in_digit = false;
        }

        tokenized.push(i.to_string());
    }

    if !digit.is_empty() {
        tokenized.push(digit);
    }

    correct_negative_numbers(tokenized)
}

fn correct_negative_numbers(tokenized: Vec<String>) -> Vec<String> {
    let mut corrected = Vec::with_capacity(tokenized.len());
    let mut i = 0;

    while i < tokenized.len() {
        if i == 0 {
            if let Ok(number) = tokenized.get(1).unwrap_or(&String::new()).parse::<f64>() {
                corrected.push((number * -1.0).to_string());
                i += 2;
                continue;
            } else {
                corrected.push(tokenized[0].to_owned());
                i += 1;
                continue;
            }
        }

        if tokenized.get(i - 1).unwrap().parse::<f64>().is_err() {
            if let Ok(number) = tokenized
                .get(i + 1)
                .unwrap_or(&String::new())
                .parse::<f64>()
            {
                corrected.push((number * -1.0).to_string());
                i += 2;
                continue;
            }
        }

        corrected.push(tokenized[i].to_owned());
        i += 1;
    }

    corrected
}
