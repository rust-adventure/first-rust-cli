use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let layout = &args[1];
    let tags = &args[2];
    let heading = &args[3];
    let filename = &args[4];

    dbg!(layout, tags, heading, filename);
}
