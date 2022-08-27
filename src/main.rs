use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    let layout = args[1].clone();
    let tags = args[2].clone();
    let heading = args[3].clone();
    let filename = args[4].clone();

    dbg!(layout, tags, heading, filename);
}
