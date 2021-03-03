// NOTE:
// This example required the "save" feature of this crate.
//

use rxscreen::Display;

fn main() {
    let d = rxscreen::Display::new(":0.0");
    if let Ok(display) = d {
        let screen = display.screenshot().unwrap();
        screen.save_as("./image.png").unwrap();
        println!("Saved to file");
    }else{
        println!("Couldn't open screen.");
    }
}
