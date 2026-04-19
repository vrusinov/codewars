// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Vladimir Rusinov

// Infix to Postfix Converter
// https://www.codewars.com/kata/52e864d1ffb6ac25db00017f/train/rust

fn precedence(op: char) -> i32 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn is_right_associative(op: char) -> bool {
    op == '^'
}

fn to_postfix(infix: &str) -> String {
    let mut output = String::new();
    let mut operators = String::new();

    for token in infix.chars() {
        if token.is_digit(10) {
            output.push(token);
        } else if token == '(' {
            operators.push(token);
        } else if token == ')' {
            while let Some(op) = operators.pop() {
                if op == '(' {
                    break;
                }
                output.push(op);
            }
        } else {
            while let Some(op) = operators.chars().last() {
                if precedence(op) > precedence(token)
                    || (precedence(op) == precedence(token) && !is_right_associative(token))
                {
                    output.push(operators.pop().unwrap());
                } else {
                    break;
                }
            }
            operators.push(token);
        }
    }

    return output + &operators.chars().rev().collect::<String>();
}

#[cfg(test)]
mod tests {
    use super::to_postfix;

    fn do_test(actual: &str, expected: &str) {
        assert_eq!(actual, expected, "\nYour answer (left) is not the correct answer (right)")
    }

    #[test]
    fn fixed_tests() {
        do_test(&to_postfix("2+7*5"), "275*+");
        do_test(&to_postfix("3*3/(7+1)"), "33*71+/");
        do_test(&to_postfix("5+(6-2)*9+3^(7-1)"), "562-9*+371-^+");
        do_test(&to_postfix("(5-4-1)+9/5/2-7/1/7"), "54-1-95/2/+71/7/-");
        do_test(&to_postfix("1^2^3"), "123^^");
    }
}
