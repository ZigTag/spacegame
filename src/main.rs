//TODO: Please organize code.

use coffee::graphics::{Color, Frame, Mesh, Point, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use std::f32::consts::PI;

const TICKS_PER_SECOND: u16 = 60;
const G: f32 = 6.743e-11;

fn main() -> Result<()> {
    MyGame::run(WindowSettings {
        title: String::from("SpaceGame"),
        size: (800, 800),
        resizable: false,
        fullscreen: false,
        maximized: false,
    })
}

struct Planet {
    color: Color,
    mass: f32,
    radius: f32,
    position: Point,
    //velocity: Point,
}

struct MyGame {
    main_planet: Planet,
    satellite_planet: Planet,
    sim_time: i32,
}

impl Game for MyGame {
    type Input = (); // No input data
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<MyGame> {
        // Load your game assets here. Check out the `load` module!
        Task::succeed(|| MyGame {
            main_planet: Planet {
                color: Color::from_rgb(66, 225, 93),
                mass: 1000.0,
                radius: 100.0,
                position: Point::new(800.0 / 2.0, 800.0 / 2.0),
                //velocity: Point::new(0.0, 0.0),
            },
            satellite_planet: Planet {
                color: Color::WHITE,
                mass: 1.0,
                radius: 10.0,
                position: Point::new(800.0 / 2.0 - 200.0, 800.0 / 2.0),
                //velocity: Point::new(0.0, 0.0),
            },
            sim_time: 0,
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        // Clear the current frame
        frame.clear(Color::BLACK);

        // Draw your game here. Check out the `graphics` module!
        let mut mesh = Mesh::new();

        mesh.fill(
            Shape::Circle {
                center: self.main_planet.position,
                radius: self.main_planet.radius,
            },
            self.main_planet.color,
        );
        mesh.fill(
            Shape::Circle {
                center: self.satellite_planet.position,
                radius: self.satellite_planet.radius,
            },
            self.satellite_planet.color,
        );

        mesh.draw(&mut frame.as_target());
    }

    fn update(&mut self, _window: &Window) {
        let tick_time = self.sim_time as f32 / TICKS_PER_SECOND as f32;
        self.sim_time += 1;

        let eccentricity: i32 = 0;

        let planetary_distance = ((self.satellite_planet.position.coords[0]
            - self.main_planet.position.coords[0])
            .powi(2)
            + (self.satellite_planet.position.coords[1] - self.main_planet.position.coords[1])
                .powi(2))
        .sqrt();

        let gravitational_parameter = G * self.main_planet.mass;

        // let velocity = gravitational_parameter / planetary_distance.powi(2);

        let p =
            (((4.0 * PI.powi(2)) * planetary_distance.powi(3)) / gravitational_parameter).sqrt();

        let semimajor_length = p / (1.0 - (eccentricity as f32 / 100.0).powi(2));

        // let time = (2.0 * PI) * (semimajor_length.powi(3) / gravitational_parameter).sqrt();

        // let sweep = 2.0 * PI / time;

        let mean_anomaly =
            (gravitational_parameter / semimajor_length.powi(3)).sqrt() * (tick_time - PI.powi(2));

        let mut eccentric_anomaly: f32 = mean_anomaly;

        for _ in 0..(eccentricity + 1) {
            eccentric_anomaly =
                mean_anomaly + ((eccentricity as f32 / 100.0) * eccentric_anomaly.sin());
        }

        let true_anomaly = 2.0
            * (((1.0 + (eccentricity as f32 / 100.0)) / 1.0 - (eccentricity as f32 / 100.0))
                .sqrt()
                * (eccentric_anomaly / 2.0).tan())
            .atan();

        let position_vector =
            planetary_distance * Point::new(true_anomaly.cos(), true_anomaly.sin());

        self.satellite_planet.position = position_vector;

        println!(
            "anmomalies {} {} {}",
            mean_anomaly, eccentric_anomaly, true_anomaly
        );
        println!("{:?}", position_vector)
    }
}
