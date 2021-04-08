extern crate printpdf;

use getopts::Options;
use printpdf::*;
use image::jpeg::JpegDecoder;
use std::fs::File;
use std::io::BufWriter;

pub struct Config {
    pub image_paths: Vec<String>,
    pub output_filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let program = args[0].clone();

        let mut opts = Options::new();
        opts.optopt("o", "output", "set output file name", "NAME");
        opts.optflag("h", "help", "print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => { m }
            Err(_) => { return Err("failed to parse options.") }
        };
        if matches.opt_present("h") {
            print_usage(&program, opts);
            return Err("help!");
        }
        let output_filename = match matches.opt_str("o") {
            Some(s) => s,
            None => String::from("output.pdf"),
        };
        let image_paths = matches.free;

        Ok(Config { image_paths, output_filename })
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    eprint!("{}", opts.usage(&brief));
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    let doc = PdfDocument::empty("printpdf graphics test");

    let dpi = 300.0;

    for image_path in config.image_paths.iter() {
        println!("image_path: {}", image_path);
        let param = ImageParameter::new(image_path.to_string()).unwrap();

        let (page_index, layer_index) = doc.add_page(Mm(param.page_width), Mm(param.page_height), "Page 2, Layer 1");
        let current_layer = doc.get_page(page_index).get_layer(layer_index);
        param.image.add_to_layer(
            current_layer.clone(),
            None,
            Some(Mm(100.0)),
            None,
            Some(param.scale),
            Some(param.scale),
            Some(dpi));
    }

    doc.save(&mut BufWriter::new(File::create(config.output_filename).unwrap())).unwrap();

    return Ok(());
}

pub struct ImageParameter {
    pub image: Image,
    pub scale: f64,
    pub page_width: f64,
    pub page_height: f64,
}
impl ImageParameter {
    pub fn new(filename: String) -> Result<ImageParameter, &'static str> {
        let mut image_file: File = File::open(filename.clone()).unwrap();
        let decoder = JpegDecoder::new(&mut image_file).unwrap();
        let image = Image::try_from(decoder).unwrap();
        let scale = 1.0; //TODO culcurate scale.

        let page_width = 210.0; //TODO portrait or landscape
        let page_height = 297.0; //TODO portrait or landscape
        Ok(ImageParameter {image, scale, page_width, page_height})
    }
}
