const SYMBOLS: [char; 8] = ['+', '-', '*', '/', '^', '(', ')', '='];

pub fn tokenize(expression: &str) -> Option<Vec<String>> {
    let mut tokenized = Vec::new();

    let mut digit = String::new();
    let mut in_digit = false;

    let mut word = String::new();
    let mut in_word = false;

    for i in expression.chars() {
        if i.is_whitespace() {
            if in_digit {
                tokenized.push(digit.clone());

                digit.clear();
                in_digit = false;
            }

            if in_word {
                tokenized.push(word.clone());

                word.clear();
                in_word = false;
            }

            continue;
        }

        if i.is_ascii_alphabetic() {
            if in_digit {
                tokenized.push(digit.clone());
                tokenized.push(String::from("*"));

                digit.clear();
                in_digit = false;
            }

            if !in_word {
                in_word = true;
            }

            word.push(i);
            continue;
        }

        if in_word {
            tokenized.push(word.clone());

            word.clear();
            in_word = false;
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

            digit.clear();
            in_digit = false;
        }

        if !SYMBOLS.contains(&i) {
            return None;
        }

        if i == '(' {
            let last = tokenized.last().unwrap_or(&String::new()).to_owned();
            let is_number = last.parse::<f64>().is_ok();
            let is_close_brac = last == ")";

            if is_number || is_close_brac {
                tokenized.push(String::from("*"));
            }
        }

        tokenized.push(i.to_string());
    }

    if !digit.is_empty() {
        tokenized.push(digit);
    }

    if !word.is_empty() {
        tokenized.push(word);
    }

    Some(correct_negative_numbers(tokenized))
}

fn correct_negative_numbers(tokenized: Vec<String>) -> Vec<String> {
    let mut corrected = Vec::with_capacity(tokenized.len());
    let mut i = 0;

    while i < tokenized.len() {
        if i == 0 && tokenized[0] == "-" {
            if let Ok(number) = tokenized.get(1).unwrap_or(&String::new()).parse::<f64>() {
                corrected.push((number * -1.0).to_string());
                i += 2;
            } else {
                corrected.push(tokenized[0].to_owned());
                i += 1;
            }

            continue;
        }

        if tokenized[i] == "-"
            && ['+', '-', '*', '/', '^']
                .contains(&tokenized.get(i - 1).unwrap().chars().next().unwrap())
        {
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
