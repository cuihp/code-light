use std::fs;

fn main() {
    let svg = include_str!("../dot_1.svg");
    println!("SVG length: {} bytes", svg.len());
    println!("Contains #E60012: {}", svg.contains("#E60012"));

    let colored = svg.replace("#E60012", "#33D933");
    let opt = resvg::usvg::Options::default();

    match resvg::usvg::Tree::from_data(colored.as_bytes(), &opt) {
        Ok(tree) => {
            println!("SVG parsed OK, size: {:?}", tree.size());
            let mut pixmap = resvg::tiny_skia::Pixmap::new(32, 32).unwrap();
            let scale = 32.0 / 1024.0;
            let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
            resvg::render(&tree, transform, &mut pixmap.as_mut());
            let data = pixmap.encode_png().unwrap();
            fs::write("/tmp/test_icon.png", &data).unwrap();
            println!("Wrote /tmp/test_icon.png ({} bytes)", data.len());

            // Check if any non-transparent pixels
            let rgba = pixmap.take();
            let opaque = rgba.chunks(4).filter(|p| p[3] > 0).count();
            println!("Opaque/semi-transparent pixels: {}/1024", opaque);
        }
        Err(e) => println!("SVG parse FAILED: {:?}", e),
    }
}
