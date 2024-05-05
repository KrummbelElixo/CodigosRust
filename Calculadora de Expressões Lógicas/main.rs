use std::io;

fn main() {
    println!("Informe a expressão");
    let mut expressao = String::new();
    io::stdin().read_line(&mut expressao).unwrap();

    while expressao.contains('*') {
        println!("{}", expressao);
        expressao = and_operator(expressao.clone());
    }

    while expressao.contains('+') {
        println!("{}", expressao);
        expressao = or_operator(expressao.clone());
    }

    while expressao.contains('>') {
        println!("{}", expressao);
        expressao = conditional_operator(expressao.clone());
    }

    while expressao.contains('#') {
        println!("{}", expressao);
        expressao = biconditional_operator(expressao.clone());
    }

    println!("{}", expressao);
}

                                                                                                                                                                                                                                                                                                                                                  
fn and_operator(expressao: String) -> String {
    let pos: usize = indice_char(&expressao, '*');
    let mut antiga_expressao = String::new();

    for(i , c) in expressao.chars().enumerate() {
        if i == pos-1 ||i == pos || i == pos+1 {
            antiga_expressao += (c.to_string()).as_str();
        }
    }

    match antiga_expressao.chars().nth(1) {
        Some('*') => {
            match expressao.chars().nth(0) == Some('V') && expressao.chars().nth(2) == Some('V') {
                true => {
                    return expressao.replace(&antiga_expressao, "V");
                }
                _ => {
                    return expressao.replace(&antiga_expressao, "F");
                }
            }
        }

        _ => {
            return "?".to_string();
        }
    }
}

fn or_operator(expressao: String) -> String {
    let pos: usize = indice_char(&expressao, '+');
    let mut antiga_expressao = String::new();

    for(i , c) in expressao.chars().enumerate() {
        if i == pos-1 ||i == pos || i == pos+1 {
            antiga_expressao += (c.to_string()).as_str();
        }
    }

    match antiga_expressao.chars().nth(1) {
        Some('+') => {
            match expressao.chars().nth(0) == Some('V') || expressao.chars().nth(2) == Some('V') {
                true => {
                    return expressao.replace(&antiga_expressao, "V");
                }
                _ => {
                    return expressao.replace(&antiga_expressao, "F");
                }
            }
        }

        _ => {
            return "?".to_string();
        }
    }
}

fn conditional_operator(expressao: String) -> String {
    let pos: usize = indice_char(&expressao, '>');
    let mut antiga_expressao = String::new();

    for(i , c) in expressao.chars().enumerate() {
        if i == pos-1 ||i == pos || i == pos+1 {
            antiga_expressao += (c.to_string()).as_str();
        }
    }

    match antiga_expressao.chars().nth(1) {
        Some('>') => {
            match expressao.chars().nth(0) == Some('V') && expressao.chars().nth(2) == Some('F') {
                true => {
                    return expressao.replace(&antiga_expressao, "F");
                }
                _ => {
                    return expressao.replace(&antiga_expressao, "V");
                }
            }
        }

        _ => {
            return "?".to_string();
        }
    }
}

fn biconditional_operator(expressao: String) -> String {
    let pos: usize = indice_char(&expressao, '#');
    let mut antiga_expressao = String::new();

    for(i , c) in expressao.chars().enumerate() {
        if i == pos-1 ||i == pos || i == pos+1 {
            antiga_expressao += (c.to_string()).as_str();
        }
    }

    match antiga_expressao.chars().nth(1) {
        Some('#') => {
            match expressao.chars().nth(0) == expressao.chars().nth(2) {
                true => {
                    return expressao.replace(&antiga_expressao, "V");
                }
                _ => {
                    return expressao.replace(&antiga_expressao, "F");
                }
            }
        }

        _ => {
            return "?".to_string();
        }
    }
}

fn indice_char(expressao: &str, letra: char) -> usize {
    let mut indice: usize = 0;

    for c in expressao.chars() {
        if letra == c {
            return indice;
        }

        indice += 1;
    }

    return indice;
}




