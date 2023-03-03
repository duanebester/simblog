use comrak::{markdown_to_html, ComrakOptions};
use std::{env, fs, path::Path};
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() == 2 { &args[1] } else { "blog" };
    let dist = if args.len() == 3 { &args[2] } else { "dist" };
    let wrapper_html = if args.len() == 4 {
        &args[3]
    } else {
        "wrapper.html"
    };

    println!("Looking in directory {}", directory);
    let wrapper_path = format!("{}/{}", directory, wrapper_html);
    let wrapper_path = Path::new(&wrapper_path);
    let wrapper = fs::read_to_string(wrapper_path).unwrap();

    _ = fs::remove_dir_all(dist);

    let mut options = ComrakOptions::default();
    options.render.unsafe_ = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());

    let entries = WalkDir::new(directory).into_iter();

    for entry in entries.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            if entry
                .file_name()
                .to_str()
                .map(|s| s.ends_with(".md"))
                .unwrap_or(false)
            {
                let out = entry
                    .path()
                    .display()
                    .to_string()
                    .replace(".md", ".html")
                    .replace(directory, dist);
                println!("writing file: {}", out);
                let md = fs::read_to_string(&entry.path()).unwrap();
                let html = markdown_to_html(&md, &options);
                let wrapped = wrapper.replace("<!--content-->", &html);
                fs::write(out, wrapped).unwrap();
            }
        } else {
            let out = entry.path().display().to_string().replace(directory, dist);
            fs::remove_dir(out.clone()).unwrap_err();
            fs::create_dir(out).unwrap();
        }
    }
    println!("Done!")
}
