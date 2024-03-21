use ico;
use image::{imageops, ImageBuffer, Rgba};
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

fn left_the_cursor<P: AsRef<Path>>(cursor_filename: P) {
    let cursor_filename = cursor_filename.as_ref();
    let file = std::fs::File::open(cursor_filename.with_extension("cur")).unwrap();
    let icon_dir = ico::IconDir::read(file).unwrap();
    icon_dir.entries().iter().for_each(|entry| {
        let iconw = entry.width();
        let iconh = entry.height();
        println!("icon size: {}x{}", iconw, iconh);
        let icon = entry.decode().unwrap();
        let (curx, cury) = icon.cursor_hotspot().unwrap();
        println!("cursor pos: {:?}", (curx, cury));

        let rgba = icon.rgba_data();
        assert_eq!(rgba.len(), (4 * iconw * iconh) as usize);
        let img = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(iconw, iconh, rgba.to_vec()).unwrap();
        let mut leftimg =
            ico::IconImage::from_rgba_data(32, 32, imageops::flip_horizontal(&img).to_vec());
        println!(
            "left cursor pos: {:?}",
            (iconw as u16 - curx - 1, cury as u16)
        );

        leftimg.set_cursor_hotspot(Some((iconw as u16 - curx - 1, cury as u16)));
        let mut lefticon_dir = ico::IconDir::new(ico::ResourceType::Cursor);

        lefticon_dir.add_entry(ico::IconDirEntry::encode(&leftimg).unwrap());
        let filename = Path::new(".").join("left").join(cursor_filename.file_name().unwrap());
        println!("{}", filename.display());
        if !filename.parent().is_none() {
            fs::create_dir_all(filename.parent().unwrap()).unwrap();
        }
        let file =
            fs::File::create(filename).unwrap();
        lefticon_dir.write(file).unwrap();
    });
}
