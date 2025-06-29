use opencv::{Result, core, highgui, imgproc, prelude::*, videoio};
use std::io::{self, Write};

mod utils;

fn main() -> Result<()> {
    let mut available_cams = Vec::new();
    println!("Detecting available cameras...");
    for i in 0..10 {
        let cam = videoio::VideoCapture::new(i, videoio::CAP_ANY)?;
        if videoio::VideoCapture::is_opened(&cam)? {
            available_cams.push(i);
        }
    }

    if available_cams.is_empty() {
        panic!("No cameras detected!");
    }

    utils::clear_terminal();

    println!("Available camera indices:");
    for cam_index in &available_cams {
        println!("  - Camera at index {}", cam_index);
    }

    print!("Please enter the camera index you want to use: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let cam_index: i32 = input
        .trim()
        .parse()
        .expect("Invalid input, expected a number");

    if !available_cams.contains(&cam_index) {
        panic!("Selected camera index {} is not available", cam_index);
    }

    let mut cam = videoio::VideoCapture::new(cam_index, videoio::CAP_ANY)?;

    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Unable to open camera at index {}", cam_index);
    }

    highgui::named_window("Camera - Light Objects", highgui::WINDOW_AUTOSIZE)?;

    loop {
        let mut frame = core::Mat::default();
        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {
            let mut gray_frame = core::Mat::default();
            imgproc::cvt_color(
                &frame,
                &mut gray_frame,
                imgproc::COLOR_BGR2GRAY,
                0,
                core::AlgorithmHint::ALGO_HINT_DEFAULT,
            )?;

            let mut thresh_frame = core::Mat::default();
            imgproc::threshold(
                &gray_frame,
                &mut thresh_frame,
                220.0,
                255.0,
                imgproc::THRESH_BINARY,
            )?;

            highgui::imshow("Camera - Light Objects", &thresh_frame)?;
        }

        if highgui::wait_key(10)? == 'q' as i32 {
            break;
        }
    }

    Ok(())
}
