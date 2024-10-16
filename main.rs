use std::io;

struct Pol {
    maximum: usize,
    start: usize,
    end: usize,
}
fn even(vector: &Vec<u8>) -> Pol {
    let mut maximum = Pol {
        maximum: 0,
        start: 1,
        end: 1,
    };
    for i in 1..vector.len() {
        let mut current = Pol {
            maximum: 0,
            start: i - 1,
            end: i,
        };
        if vector[i] == vector[i - 1] {
            current.maximum += 2;
            let mut u = 1;
            while i>u && i + u < vector.len() {
                if vector[i - u - 1] == vector[i + u] {
                    current.end += 1;
                    current.start -= 1;
                    current.maximum += 2;
                    u += 1;
                } else {
                    break;
                }
            }
        }
        if current.maximum > maximum.maximum {
            maximum = current
        };
    }
    maximum
}
fn uneven(vector: &Vec<u8>) -> Pol {
    let mut maximum = Pol {
        maximum: 1,
        start: 0,
        end: 0,
    };
    for i in 1..vector.len() {
        let mut current = Pol {
            maximum: 0,
            start: i,
            end: i,
        };
        let mut u = 1;
        while u<vector.len()-i && 1+i>u  {
            if vector[i - u] == vector[i + u] {
                current.end += 1;
                current.start -= 1;
                current.maximum += 2;
                u += 1;
            } else {
                break;
            }
        }
        if current.maximum > maximum.maximum {
            maximum = current
        };
    }
    maximum
}
fn main() {
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).expect("impossible");
    let vector: Vec<u8> = line.chars().map(|x| x as u8).collect();
    let even_pol = even(&vector);
    let uneven_pol =uneven(&vector);
    if line.len() > 1 {
        if even_pol.maximum > uneven_pol.maximum {
            for i in even_pol.start..even_pol.end+1 {
                print!("{}", vector[i] as char);
            }
        } else {
            for i in uneven_pol.start..uneven_pol.end+1 {
                print!("{}", vector[i] as char);
            }
        }
    }
    println!("");
}
