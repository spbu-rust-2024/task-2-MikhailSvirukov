use std::io;

fn main() {
    let mut line:String=String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("impossible");
    let vector:Vec <u8>= line.chars().map(|x| x as u8).collect();
    let length = vector.len();
    let half = length/2;
    let mut start=0;
    let mut finish=0;
    let mut maximum=1;


    if length==1 {print!(""); return}

    for i in 1..half {
        let mut current=1;
        let mut cur_start=i;
        let mut cur_finish=i;
        for u in 1..i+1 {
            if vector[i-u]==vector[i+u] {
                cur_finish=cur_finish+1;
                cur_start=cur_start-1;
                current=current+2;
            }else {
                break;
            }
        }
        if current>maximum {maximum=current; start=cur_start; finish=cur_finish;}
    }

    for i in half..length {
        let mut current=1;
        let mut cur_start=i;
        let mut cur_finish=i;
        for u in 1..length-i {
            if vector[i-u]==vector[u+i] {
                cur_finish=cur_finish+1;
                cur_start=cur_start-1;
                current=current+2;
            } else {
                break;
            }
        }
        if current>maximum {maximum=current; start=cur_start; finish=cur_finish;}
    }

    for i in 1..half {
        let mut cur_start=i-1;
        let mut cur_finish=i;
        let mut current=0;
        if vector[i]==vector[i-1] {
            current=current+2;
            for u in 1..i {
                if vector[i-u-1]==vector[i+u] {
                    cur_finish=cur_finish+1;
                    cur_start=cur_start-1;
                    current=current+2;
                } else {
                    break;
                }
            }
            if current>maximum {maximum=current; start=cur_start; finish=cur_finish}
        }
    }

    for i in half..length{
        let mut cur_start=i-1;
        let mut cur_finish=i;
        let mut current=0;
        if vector[i]==vector[i-1] {
            current=current+2;
            for u in 1..length-i-1 {
                if vector[i-1-u]==vector[i+u] {
                    current=current+2;
                    cur_finish=cur_finish+1;
                    cur_start=cur_start-1;
                } else {
                    break;
                }
            }
            if current>maximum {maximum=current; start=cur_start; finish=cur_finish;}
        }
    }

    if length>1 {
        for i in start..finish+1 {
            print!("{}", vector[i] as char)
        }
    }
}
