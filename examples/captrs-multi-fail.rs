use std::{thread::{self, sleep}, time::Duration};

use captrs::{Bgr8, CaptureError};

fn main() {
    let handles: Vec<_> = (0..3).map(|i| {
        thread::spawn(move || {
            match captrs::Capturer::new(i) {
                Ok(mut capturer) => {
                    let (w, h) = capturer.geometry();
                    let size = w as u64 * h as u64;
                    loop {
                        let ps = capturer.capture_frame();
                        // if timeout, continue loop
                        let ps = match ps {
                            Ok(ps) => ps,
                            Err(CaptureError::Timeout) => continue,
                            Err(e) => {
                                println!("Error: {:?}", e);
                                break;
                            }
                        };
                
                        let (mut tot_r, mut tot_g, mut tot_b) = (0, 0, 0);
                
                        for Bgr8 { r, g, b, .. } in ps.into_iter() {
                            tot_r += r as u64;
                            tot_g += g as u64;
                            tot_b += b as u64;
                        }
                
                        println!("{} Avg: {:?}",i, (tot_r / size, tot_g / size, tot_b / size));
                
                        sleep(Duration::from_millis(80));
                    }
                },
                Err(e) => {
                    println!("{} Failed to initialize capturer: {:?}", i, e);
                }
            }
        })
    }).collect();

    for handle in handles {
        match handle.join() {
            Ok(_) => println!("Thread finished successfully."),
            Err(e) => println!("Thread panicked: {:?}", e),
        }
    }
}