use clap::Parser;
use leftcursor::{ani, cursor::left_the_cursor};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "leftcursor")]
#[command(author = "owlright")]
#[command(version = "v0.1.0")]
#[command(about = "generate left-handed cursors", long_about = None)]
struct Cli {
    // 注意下面的注释是三个斜杠!!!
    /// Provide the full path to the directory containing the cursor files
    #[arg(short, long)]
    input_dir: Option<String>,

    /// Provide the full path to the directory containing the cursor files
    #[arg(short, long)]
    output_dir: Option<String>,

    /// Give a prefix to the generated cursor pack
    prefix: Option<String>,
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
fn main() {
    let file = fs::File::open("resource/Windows-Cursor-Concept-v2.0/light/default/working.ani").unwrap();
    let ani = ani::Ani::read(file).unwrap();

    std::process::exit(0);
    let cli = Cli::parse();
    let input_dir_name = cli.input_dir.unwrap_or("".to_string());
    let out_dir_name = cli.output_dir.unwrap_or("./left".to_string());
    let perfix_name = cli.prefix.unwrap_or("".to_string());
    if input_dir_name.is_empty() {
        println!("Please provide a directory name!");
        return;
    }
    let cursor_paths = fs::read_dir(Path::new(&input_dir_name))
        .unwrap_or_else(|_| panic!("can't open dir {}", &input_dir_name));
    for path in cursor_paths {
        let path = path.unwrap().path();
        println!("{}", path.display());
        if let Some(ext) = path.extension() {
            if ext == "cur" {
                left_the_cursor(path, PathBuf::from(&out_dir_name), &perfix_name);
            }
        }
    }
}
