extern crate printpdf;

use printpdf::*;
use std::io::Cursor;
use image::jpeg::JpegDecoder;
use std::fs::File;
use std::io::BufWriter;

fn main() {
    let (doc, page1, layer1) = PdfDocument::new("printpdf graphics test", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);

    // currently, the only reliable file formats are bmp/jpeg/png
    // this is an issue of the image library, not a fault of printpdf

    let mut image_file = File::open("sample.jpg").unwrap();
    let decoder = JpegDecoder::new(&mut image_file).unwrap();
    let image2 = Image::try_from(decoder).unwrap();

    // layer,     
    image2.add_to_layer(current_layer.clone(), None, None, None, None, None, None);

    doc.save(&mut BufWriter::new(File::create("test_image.pdf").unwrap())).unwrap();
    println!("Hello, world!");
}
