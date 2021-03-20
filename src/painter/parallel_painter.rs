use crate::brush::Brush;
use crate::canvas::Canvas;
use crate::painter::Painter;

use std::vec::Vec;
use std::thread;
use std::sync::{mpsc, Arc};
use image::{RgbaImage};

pub struct ParallelPainter {}

fn generate_vec(brush: Arc<dyn Brush>, canvas: Arc<dyn Canvas>, i: usize, max: usize) -> Vec<u8> {
    let i = i as u32;
    let max = max as u32;

    let (width, height) = (canvas.width(), canvas.height());
    let start = (height * i) / max;
    let end =
        if i == max - 1 { height }
        else { (height * (i+1)) / max };

    let num_elements = (end-start)*height*4;
    let mut pixels = std::vec::Vec::with_capacity(num_elements as usize);

    for y in start..end {
        for x in 0..width {
            let (r, g, b) = brush.color(x, y);
            pixels.push(r);
            pixels.push(g);
            pixels.push(b);
            pixels.push(255);
        }
    }

    pixels
}

impl Painter for ParallelPainter {
    fn paint(&self, brush: Arc<dyn Brush>, canvas: Arc<dyn Canvas>) {
        let (tx, rx) = mpsc::channel();

        let mut threads = vec![];

        let max: usize = num_cpus::get();

        for i in 0..max {
            let brush = brush.clone();
            let canvas = canvas.clone();
            let tx = tx.clone();
            threads.push(thread::spawn(move || {
                let segment = generate_vec(brush, canvas, i, max);
                tx.send((i, segment)).expect("Could not send!");
            }));
        }
        
        let mut pixels = vec![vec![]; max as usize];

        for t in threads {
            t.join().expect("Could not join threads!");
            let (i, segment) = rx.recv().expect("Channel was closed!");
            pixels[i] = segment;
        }

        let pixels = pixels.concat();
        let img = RgbaImage::from_vec(canvas.width(), canvas.height(), pixels).expect("Failed to create image!");
        canvas.display(img);
    }
}
