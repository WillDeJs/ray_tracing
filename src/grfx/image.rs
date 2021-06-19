use crate::grfx::color::Color;
use pixels::{Pixels, SurfaceTexture};
use std::fs::File;
use std::io::Write;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

#[allow(dead_code)]
impl Image {
    ///
    /// create a new image from a vector of Colors and with the given width and height dimensions
    ///
    pub fn from_colors(width: u32, height: u32, colors: Vec<Color>) -> Self {
        Self {
            width: width,
            height: height,
            pixels: colors,
        }
    }

    ///
    /// Show image of the screen using winit and pixels
    ///
    /// Important: This method call consumes/moves the Image.
    ///
    /// Therefore it annot be used after this method is called.
    ///
    pub fn show(self) {
        let event_loop = EventLoop::new();
        let image_size = LogicalSize::new(self.width, self.height);
        let window = WindowBuilder::new()
            .with_inner_size(image_size)
            .with_title("Ray Tracing example")
            .build(&event_loop)
            .unwrap();
        let mut input = WinitInputHelper::new();
        let surface_texture = SurfaceTexture::new(self.width, self.height, &window);
        let mut pixelbuffer = Pixels::new(self.width, self.height, surface_texture).unwrap();

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => *control_flow = ControlFlow::Exit,
                Event::RedrawRequested(_windowid) => {
                    for (i, pixel) in pixelbuffer.get_frame().chunks_exact_mut(4).enumerate() {
                        pixel.copy_from_slice(&self.pixels.get(i).unwrap().as_bytes());
                    }
                    if pixelbuffer.render().is_err() {
                        println!("pixels.render() failed");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                _ => (),
            }
            if input.update(&event) {
                window.request_redraw();
            }
        });
    }

    ///
    /// Write image to a file with the name/location of filename
    /// Returns () upon success or io error in case of ailure.
    ///
    /// Export format Portable Pixel Map (PPM)
    pub fn write_image(&self, filename: &str) -> Result<(), std::io::Error> {
        let mut file = File::create(filename).expect("Could not create file");
        file.write_fmt(format_args!(
            "{}\n{} {} \n255\n",
            "P3", self.width, self.height
        ))
        .unwrap();
        for color in &self.pixels {
            file.write_all(color.to_string().as_bytes())?;
        }
        Ok(())
    }
}
