use std::{env, fs::File, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    let layout = &args[1];
    let tags = &args[2];
    let heading = &args[3];
    let filename = &args[4];

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
