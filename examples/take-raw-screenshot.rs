use rxscreen::Display;

fn main() {
    let d = rxscreen::Display::new(":0.0");
    if let Ok(display) = d {
        let screen = display.capture().unwrap();
        println!("Screenshot is {}bytes long", unsafe { screen.as_raw_slice().len() });
    }else{
        println!("Couldn't open screen.");
    }
}
