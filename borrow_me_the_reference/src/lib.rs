pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut num_minus = 0;
    let mut num_plus = 0;

    for c in s.chars() {
        match c {
            '-' => {
                // Compter le nombre de '-' consécutifs.
                num_minus += 1;
            }
            '+' => {
                // Compter le nombre de '+' consécutifs.
                num_plus += 1;
            }
            _ => {
                // Traiter les caractères lorsque nous n'avons pas de '-' ou '+' consécutifs.
                if num_minus > 0 {
                    // Supprimer les caractères en remontant si nous avons des '-' consécutifs.
                    let len = result.len();
                    if len > num_minus {
                        result.truncate(len - num_minus);
                    } else {
                        result.clear();
                    }
                    num_minus = 0;
                }
                if num_plus == 0 {
                    // Ajouter le caractère seulement si nous n'avons pas de '+' consécutifs restants.
                    result.push(c);
                } else {
                    // Décrémenter la variable '+'.
                    num_plus -= 1;
                }
            }
        }
    }

    // Supprimer les caractères restants en remontant s'il y a des '-' consécutifs à la fin.
    let len = result.len();
    if len > num_minus {
        result.truncate(len - num_minus);
    } else {
        result.clear();
    }

    *s = result;
}

pub fn do_operations(v: &mut Vec<String>) {
    for equation in v.iter_mut() {
        let parts: Vec<&str> = equation.split(|c| c == '+' || c == '-').collect();
        let operator = equation.trim_matches(char::is_numeric);

        if parts.len() == 2 {
            let left = parts[0].trim().parse::<i32>().unwrap();
            let right = parts[1].trim().parse::<i32>().unwrap();
            let result = match operator {
                "+" => left + right,
                "-" => left - right,
                _ => panic!("Invalid operator"),
            };
            *equation = result.to_string();
        } else {
            panic!("Invalid equation format: {}", equation);
        }
    }
}