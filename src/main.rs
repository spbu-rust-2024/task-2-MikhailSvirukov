use std::io;

fn main() {
    let mut line:String=String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("impossible");
    let vector = line.as_bytes();
    let mut start=0;
    let mut finish=0;
    let mut maximum=1;

    for i in 1..(vector.len())/2 {
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

    for i in (vector.len())/2..vector.len() {
        let mut current=1;
        let mut cur_start=i;
        let mut cur_finish=i;
        for u in 1..vector.len()-i {
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

    for i in 1..vector.len()/2 {
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

    for i in vector.len()/2..vector.len()-1 {
        let mut cur_start=i-1;
        let mut cur_finish=i;
        let mut current=0;
        if vector[i]==vector[i-1] {
            current=current+2;
            for u in 1..vector.len()-i-1 {
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


    print!("{}", &line[start..finish+1]);
}
