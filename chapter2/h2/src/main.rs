use h2::*;


fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 100x756 -1.20,0.35 -1,0.20", args[0]);
        
        std::process::exit(0);
    }

    let bounds = units::SquareBoundsPx::from(
        utils::parse_pair(&args[2], 'x').expect("Error parsing image dimensions")
    );
    let upper_left = 
        utils::parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = 
        utils::parse_complex(&args[4]).expect("Error parsing lower right corner point");

    let mut pixels = vec![0; bounds.width * bounds.height];
    
    graphics::render(&mut pixels, bounds, upper_left, lower_right);
    graphics::write_image(&args[1], &pixels, &bounds)
        .expect("Error writing PNG file");
}
