#[allow(warnings)]
mod bindings;

use bindings::exports::gaku101::glitch_art::png_glitchable::{Guest, ScanLine};
use rand::Rng;

struct Component;

impl Guest for Component {  
    fn glitch(mut scan_line: ScanLine) -> ScanLine {
        let mut rng = rand::rng();
        scan_line.pixel_data[0] = rng.random_range(0..=255);
        scan_line
    }
}

bindings::export!(Component with_types_in bindings);
