extern crate printpdf;
extern crate image;

use getopts::Options;
use printpdf::*;
use image::jpeg::JpegDecoder;
use image::DynamicImage;
use std::fs::File;
use std::path::Path;
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
            return Err("");
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
    let brief = format!("Usage: {} [options] FILE [FILE..]", program);
    eprint!("{}", opts.usage(&brief));
}

pub fn run(config: Config) -> Result<(), Box<Error>> {

    let doc = PdfDocument::empty("pdfpackman");

    let dpi = 72.0;

    for image_path in config.image_paths.iter() {
        println!("reading image: {}", image_path);
        let param = ImageParameter::new(image_path.to_string(), dpi).unwrap();

        let (page_index, layer_index) = doc.add_page(Mm(param.page_width), Mm(param.page_height), "image");
        let current_layer = doc.get_page(page_index).get_layer(layer_index);
        param.image.add_to_layer(
            current_layer.clone(),
            Some(Mm(param.position.x)),
            Some(Mm(param.position.y)),
            None,
            Some(param.scale),
            Some(param.scale),
            Some(dpi));
    }

    let doc = doc.with_conformance(PdfConformance::Custom(CustomPdfConformance {
        requires_icc_profile: false,
        requires_xmp_metadata: false,
        .. Default::default()
    }));
    doc.save(&mut BufWriter::new(File::create(&config.output_filename).unwrap())).unwrap();
    println!("saved {}", &config.output_filename);

    return Ok(());
}

#[derive(Debug, PartialEq)]
pub struct Position {
    x: f64,
    y: f64,
}

#[derive(Debug)]
pub struct ImageParameter {
    pub image: Image,
    pub scale: f64,
    pub position: Position,
    pub page_width: f64,
    pub page_height: f64,
    pub dpi: f64,
}

impl ImageParameter {
    pub fn new(filename: String, dpi: f64) -> Result<ImageParameter, &'static str> {
        let mut image_file: File = File::open(filename.clone()).unwrap();
        let ext = Path::new(&filename).extension().unwrap().to_str().unwrap().to_lowercase();
        let image = match &ext[..] {
            "jpg" => Image::try_from(JpegDecoder::new(&mut image_file).unwrap()).unwrap(),
            "jpeg" => Image::try_from(JpegDecoder::new(&mut image_file).unwrap()).unwrap(),
            "png" => {
                let image = image::open(&filename).unwrap().to_rgb8();
                Image::from_dynamic_image(&DynamicImage::ImageRgb8(image))
            },
            _ => return Err("failed to convert. "),
        };
        let width = Mm::from(image.image.width.into_pt(dpi)).0;
        let height = Mm::from(image.image.height.into_pt(dpi)).0;
        let scale_calculator = ScaleCalculator {width, height};
        let orientation = scale_calculator.get_orientation();
        let scale = scale_calculator.get_scale(&orientation);
        let position = scale_calculator.get_position(&orientation, scale);

        let page_width = match orientation {
            Orientation::Landscape {width, height: _} => width,
            Orientation::Portrait {width, height: _} => width,
        };
        let page_height = match orientation {
            Orientation::Landscape {width: _, height} => height,
            Orientation::Portrait {width: _, height} => height,
        };
        Ok(ImageParameter {image, scale, position, page_width, page_height, dpi})
    }
}

#[derive(Debug, PartialEq)]
enum Orientation {
    Landscape {width: f64, height: f64},
    Portrait {width: f64, height: f64},
}

#[derive(Debug)]
struct ScaleCalculator {
    width: f64,
    height: f64,
}
impl ScaleCalculator {
    fn get_orientation(&self) -> Orientation {
        let diff = self.width - self.height;
        if diff > 0.0 {
            return Orientation::Landscape {width: 297.0, height: 210.0};
        }
        return Orientation::Portrait {width: 210.0, height: 297.0};
    }

    fn get_scale(&self, orientation: &Orientation) -> f64 {
        let w = match orientation {
            Orientation::Landscape {width, height: _} => width / self.width,
            Orientation::Portrait {width, height: _} => width / self.width,
        };
        let h = match orientation {
            Orientation::Landscape {width: _, height} => height / self.height,
            Orientation::Portrait {width: _, height} => height / self.height,
        };
        if  w < h {
            return w;
        }
        return h;
    }

    fn get_position(&self, orientation: &Orientation, scale: f64) -> Position {
        let x = match orientation {
            Orientation::Landscape {width, height: _} => (width - (self.width * scale)) / 2.0,
            Orientation::Portrait {width, height: _} => (width - (self.width * scale)) / 2.0,
        };
        let y = match orientation {
            Orientation::Landscape {width: _, height} => (height - (self.height * scale)) / 2.0,
            Orientation::Portrait {width: _, height} => (height - (self.height * scale)) / 2.0,
        };
        return Position {x, y};
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_calculator_init() {
        let c: ScaleCalculator = ScaleCalculator {width: 1.0, height: 2.0};
        assert_eq!(c.width, 1.0);
        assert_eq!(c.height, 2.0);
    }

    #[test]
    fn scale_calculator_orientation_landscape() {
        let c: ScaleCalculator = ScaleCalculator {width: 100.0, height: 50.0};
        assert_eq!(c.get_orientation(), Orientation::Landscape {width: 297.0, height: 210.0});
    }

    #[test]
    fn scale_calculator_orientation_portrait() {
        let c: ScaleCalculator = ScaleCalculator {width: 100.0, height: 150.0};
        assert_eq!(c.get_orientation(), Orientation::Portrait {width: 210.0, height: 297.0});
    }

    #[test]
    fn scale_calculator_orientation_landscape_scale() {
        let c: ScaleCalculator = ScaleCalculator {width: 100.0, height: 100.0};
        let o = Orientation::Landscape {width: 297.0, height: 210.0};
        let expected = 210.0 / 100.0;
        assert_eq!(c.get_scale(&o), expected);
    }

    #[test]
    fn scale_calculator_position() {
        let c: ScaleCalculator = ScaleCalculator {width: 50.0, height: 50.0};
        let o = Orientation::Landscape {width: 200.0, height: 100.0};
        let expected = Position {x: 50.0, y: 0.0};
        assert_eq!(c.get_position(&o, 2.0), expected);
    }

    #[test]
    fn image_parameter_init_jpg() {
        let param = ImageParameter::new("sample/sample-jpg.jpg".to_string(), 72.0).unwrap();
        assert_eq!(param.dpi, 72.0);
    }

    #[test]
    fn image_parameter_init_jpg_ext_uppercase() {
        let param = ImageParameter::new("sample/sample-jpg-ext.JPG".to_string(), 72.0).unwrap();
        assert_eq!(param.dpi, 72.0);
    }

    #[test]
    fn image_parameter_init_png() {
        let param = ImageParameter::new("sample/sample-png.png".to_string(), 72.0).unwrap();
        assert_eq!(param.dpi, 72.0);
    }

}
