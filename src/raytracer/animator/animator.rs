use raytracer::animator::Interpolator;
use raytracer::Renderer;
use scene::{AnimatedCamera, Scene};
use std::sync::mpsc::sync_channel;
use std::sync::Arc;
use std::thread;

pub struct Animator {
    pub fps: f64,
    pub animate_from: f64, // Number of frames is rounded down to nearest frame
    pub animate_to: f64,
    pub starting_frame_number: u32, // For filename
    pub renderer: Renderer,
    pub interpolator: Box<Interpolator>,
}

// TODO: Non-linear interpolation
impl Animator {
    // TODO: make this a Surface iterator so both single frame and animation
    // process flows are similar
    pub fn animate(&self, camera: &AnimatedCamera, shared_scene: Arc<Scene>, filename: &str) {
        let animate_start = ::time::get_time();
        let length = self.animate_to - self.animate_from;
        let total_frames = (self.fps * length).floor() as u32;

        // Allow one frame to be renderered while the previous one is being written
        let (frame_tx, frame_rx) = sync_channel(0);
        let (exit_tx, exit_rx) = sync_channel(0);

        let starting_frame_number = self.starting_frame_number;

        let filename = filename.to_string();
        thread::spawn(move || {
            for (frame_num, frame_data) in frame_rx.iter().enumerate() {
                let file_frame_number = starting_frame_number as usize + frame_num;

                let shared_name = format!("{}{:06}.ppm", filename, file_frame_number);
                ::util::export::to_ppm(frame_data, &shared_name);
            }

            exit_tx.send(()).unwrap();
        });

        for frame_number in 0..total_frames {
            let time = self.animate_from + frame_number as f64 / self.fps;
            let lerped_camera = self.interpolator.interpolate(&camera, time);
            let frame_data = self.renderer.render(lerped_camera, shared_scene.clone());
            frame_tx.send(frame_data).unwrap();

            ::util::print_progress("*** Frame", animate_start.clone(), frame_number as usize + 1usize, total_frames as usize);
            println!("");
        }
        drop(frame_tx);

        let () = exit_rx.recv().unwrap();
    }
}
