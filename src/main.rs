use leftcursor::cursor::left_the_cursor;
use std::fs;
use std::path::Path;

fn main() {
    let cursor_paths = fs::read_dir(
        Path::new("resource")
            .join("capitaine-cursors")
            .join(".windows"),
    )
    .expect("can't read dir");
    for path in cursor_paths {
        let path = path.unwrap().path();
        println!("{}", path.display());
        if let Some(ext) = path.extension() {
            if ext == "cur" {
                left_the_cursor(path);
            }
        }
    }
}
