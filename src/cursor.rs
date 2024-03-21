use ico;
use image::{imageops, ImageBuffer, Rgba};
use std::{fs, io::Write, path::Path};

pub fn left_the_cursor<P: AsRef<Path>>(cursor_filename: P, output_dir: P, prefix: &str) {
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
        let filename = Path::new(output_dir.as_ref())
            .join(prefix.to_string() + cursor_filename.file_name().unwrap().to_str().unwrap());
        println!("{}", filename.display());
        if !filename.parent().is_none() {
            fs::create_dir_all(filename.parent().unwrap()).unwrap();
        }
        let file = fs::File::create(filename).unwrap();
        lefticon_dir.write(file).unwrap();
    });
}

pub fn generate_installinf() {
    let mut installinf = match fs::File::create("tmp/install.inf") {
        Ok(f) => f,
        Err(e) => panic!("couldn't create install.inf: {}", e),
    };
    let config = "[Version]\nsignature=\"$CHICAGO$\"";
    installinf.write_all(config.as_bytes()).expect("couldn't write to install.inf");
}
