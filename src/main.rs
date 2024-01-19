use std::{env::args, process::ExitCode, fs::File, io::Read};

struct Point {
    x: f64,
    y: f64,
}

fn main() -> ExitCode {
    let mut args = args();
    args.next();
    let f = match args.next() {
        Some(f) => {f}
        None => {
            eprintln!("Provide input file");
            return 1.into();
        }
    };

    let mut values = vec![];
    let mut cont = Default::default();
    match File::open(f) {
        Ok(mut f) => match f.read_to_string(&mut cont){
            Ok(_) => (),
            Err(e) => {eprintln!("error reading file: {e}"); return 1.into();}
        }
        Err(e) => {eprintln!("error reading file: {e}"); return 1.into();}
    };

    for (i, line) in cont.lines().enumerate() {
        let line = line.trim();
        if line.len() == 0 {continue}
        let (x, y) = line.split_at(match line.find(","){
            Some(v) => v,
            None => {eprintln!("failed parsing line {}", i+1); return 1.into();}
        });
        let (_, y) = y.split_at(1);
        let y = y.trim();

        let (x, y) = (x.parse::<f64>(), y.parse::<f64>());
        let (x, y) = match (x, y) {
            (Ok(x), Ok(y)) => (x, y),
            _ => {eprintln!("error parsing values at line {}", i+1); return 1.into();}
        };
        values.push(Point{x, y});
    }

    let mut coeffs: Vec<f64> = Vec::with_capacity(values.len());
    for v in &values {
        if v.y != 0. {
            let mut product = 1.;
            for a in &values {
                if v.x != a.x { product *= v.x - a.x; }
            }
            coeffs.push(v.y / product)
        } else {coeffs.push(0.);}
    }
    print!("f(x) = ");

    let mut paren = false;
    for v in &values {
        if v.y == 0. {
            paren = true;
            if v.x != 0. {print!("(x-{})", v.x);}
            else {print!("x");}
        }
    }if paren {print!("(")};

    let mut first = true;
    for (i, a) in (&coeffs).into_iter().enumerate() {
        if *a != 0. {
            if !first && *a > 0. {print!(" +")}
            first = false;

            print!(" {a}");
            for (j, v) in (&values).into_iter().enumerate() {
                if i != j {
                    if v.y == 0. {}
                    else if v.x != 0. {print!("(x-{})", v.x);}
                    else {print!("x");}
                }
            }
        }
    }if paren {print!(")")};
    println!();

    0.into()
}
