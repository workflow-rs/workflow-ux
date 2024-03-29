use crate::error::error;
use crate::prelude::*;
use crate::result::Result;
//use qrcodegen::Mask;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;
//use qrcodegen::QrSegment;
//use qrcodegen::Version;
pub struct SVGData {
    pub data: String,
    pub finder: String,
    pub logo_start: f32,
    pub logo_size: u32,
}

#[derive(Clone)]
pub struct Colors {
    pub background: String,
    pub data: String,
    pub finder: String,
}
impl Default for Colors {
    fn default() -> Self {
        Self {
            background: "#FFFFFF".to_string(),
            data: "#000000".to_string(),
            finder: "#000000".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Options {
    pub border: u16,
    pub ecl: QrCodeEcc,
    pub logo_size: u8,
    pub logo: Option<String>,
    pub colors: Option<Colors>,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            border: 4,
            ecl: QrCodeEcc::High,
            logo_size: 20,
            logo: None,
            colors: None,
        }
    }
}

impl Options {
    pub fn from_attributes(attributes: &Attributes) -> Result<Self> {
        let mut options = Self::default();
        if let Some(border) = attributes.get("qr_border") {
            let border: u16 = border.parse()?;
            options.border = border;
        }

        if let Some(logo_size) = attributes.get("qr_logo_size") {
            options.logo_size = logo_size.parse()?;
        }

        let mut colors = Colors::default();

        if let Some(color) = attributes.get("qr_bg_color") {
            colors.background = color.clone();
        }
        if let Some(color) = attributes.get("qr_data_color") {
            colors.data = color.clone();
        }
        if let Some(color) = attributes.get("qr_finder_color") {
            colors.finder = color.clone();
        }

        if let Some(logo) = attributes.get("qr_logo") {
            options.logo = Some(logo.clone());
        }

        options.colors = Some(colors);

        if let Some(ecl) = attributes.get("ecl") {
            match ecl.to_lowercase().as_str() {
                "low" => options.ecl = QrCodeEcc::Low,
                "medium" => options.ecl = QrCodeEcc::Medium,
                "quartile" => options.ecl = QrCodeEcc::Quartile,
                "high" => options.ecl = QrCodeEcc::High,
                _ => {}
            }
        }

        Ok(options)
    }

    pub fn has_logo(&self) -> bool {
        self.logo.is_some()
    }
}

pub fn text_to_qr(text: &str) -> Result<String> {
    let options = Options::default();
    let svg = text_to_qr_with_options(text, &options)?;
    Ok(svg)
}

pub fn text_to_qr_with_options(text: &str, options: &Options) -> Result<String> {
    let qr = QrCode::encode_text(text, options.ecl)?;
    let svg = qr_to_svg(&qr, options)?;
    Ok(svg)
}

pub fn qr_to_svg(qr: &QrCode, options: &Options) -> Result<String> {
    let mut svg = String::new();
    svg.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\">");
    svg.push_str("<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">");

    let size = qr.size();
    let view_size = size
        .checked_add(options.border.checked_mul(2).unwrap() as i32)
        .unwrap();
    svg.push_str(
        &format!("<svg width=\"100%\" height=\"100%\" viewBox=\"0 0 {view_size} {view_size}\" version=\"1.1\" 
    xmlns=\"http://www.w3.org/2000/svg\">"));

    let default_colors = Colors::default();
    let colors = options.colors.as_ref().unwrap_or(&default_colors);

    svg.push_str(&format!(
        "<rect width=\"100%\" height=\"100%\" fill=\"{}\" />",
        colors.background
    ));

    let mut logo_size = None;
    if options.has_logo() {
        logo_size = Some(options.logo_size);
    }

    let info = qr_svg_path_data(qr, options.border, logo_size)?;
    svg.push_str(&format!(
        "<path d=\"{}\" fill=\"{}\" />",
        info.data, colors.data
    ));
    svg.push_str(&format!(
        "<path d=\"{}\" fill=\"{}\" />",
        info.finder, colors.finder
    ));

    if let Some(img) = &options.logo {
        svg.push_str(&format!(
            "<image href=\"{}\" x=\"{1}\" y=\"{1}\" height=\"{2}\" width=\"{2}\" />",
            img, info.logo_start, info.logo_size
        ));
    }

    svg.push_str("</svg>");

    Ok(svg)
}

pub fn qr_svg_path_data(qr: &QrCode, border: u16, logo_size: Option<u8>) -> Result<SVGData> {
    let border = border as i32;
    let mut data = String::new();
    let mut finder = String::new();

    let size = qr.size();
    let mut logo_start = 0;
    let mut logo_end = 0;
    let mut with_logo = false;
    if let Some(logo_size_percent) = logo_size {
        //let logo_size_ratio = 30;//20 percent;
        if logo_size_percent > 30 {
            return Err(error!("QR logo size cant be more than 30%"));
        }
        let logo_size_percent = logo_size_percent as i32;
        with_logo = true;
        let logo_size = size * logo_size_percent / 100;
        let size_half = size / 2;
        let logo_half = logo_size / 2;
        logo_start = size_half - logo_half;
        logo_end = logo_start + logo_size;
    }

    //println!("size:{size}, border:{border}");
    //log_trace!("logo_start:{logo_start}, logo_end:{logo_end}");

    for y in 0..size {
        for x in 0..size {
            if !qr.get_module(x, y) {
                continue;
            }
            let is_finder = (0..7).contains(&x) && (0..7).contains(&y)
                || (size - 7..size).contains(&x) && (0..7).contains(&y)
                || (0..7).contains(&x) && (size - 7..size).contains(&y);
            if is_finder {
                if x != 0 || y != 0 {
                    finder += " ";
                }
                finder += &format!("M{},{}h1v1h-1z", x + border, y + border);
            } else if with_logo
                && y >= logo_start
                && y <= logo_end
                && x >= logo_start
                && x <= logo_end
            {
                //
                //log_trace!("x:{x}, y:{y}");
            } else {
                if x != 0 || y != 0 {
                    data += " ";
                }
                data += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }

    let logo_size = (logo_end - logo_start) as u32;
    let mut logo_start = logo_start as f32 + border as f32;
    //if logo_start%2.0 == 0.0{
    logo_start += 0.5;
    //}
    //log_trace!("size:{size}, logo_start:{logo_start}, logo_size: {logo_size}");
    Ok(SVGData {
        data,
        finder,
        logo_start,
        logo_size,
    })
}
