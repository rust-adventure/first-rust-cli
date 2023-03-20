use std::{env, fs::File, io::Write};

fn main() {
    let mut args = env::args();
    // skip the program name argument
    args.next();

    let layout = args
        .next()
        .expect("The first argument must be the layout but there was no value.");
    let tags = args
        .next()
        .expect("The second argument must be the tags but there was no value.");
    let heading = args
        .next()
        .expect("The third argument must be the heading but there was no value.");
    let filename = args
        .next()
        .expect("The fourth argument must be the filename but there was no value.");

    let new_file_contents = format!(
        "---
layout: {layout}
tags: {tags}
status: draft
---

# {heading}

"
    );

    let mut file = File::create(filename).unwrap();
    file.write_all(new_file_contents.as_bytes()).unwrap();
}
