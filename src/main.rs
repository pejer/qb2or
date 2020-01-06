use std::env;
mod qb2or;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage {:?} <path to image file>", args[0]);
        return;
    }
    let qb2or_bitmap = qb2or::new(&args[1]);
    print!("{}",qb2or_bitmap.parse());
}
