use coffee::graphics::{Color, Frame, Mesh, Point, Shape, Window, WindowSettings};
use coffee::load::Task;
use coffee::{Game, Result, Timer};

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
}

struct MyGame {
    main_planet: Planet,
    satellite_planet: Planet,
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
            },
            satellite_planet: Planet {
                color: Color::WHITE,
                mass: 1.0,
                radius: 10.0,
                position: Point::new(800.0 / 2.0 - 200.0, 800.0 / 2.0),
            },
        })
    }

    fn update(&mut self, _window: &Window) {
        let r = ((self.satellite_planet.position.coords[0] - self.main_planet.position.coords[0])
            .powi(2)
            + (self.satellite_planet.position.coords[1] - self.main_planet.position.coords[1])
                .powi(2))
        .sqrt();

        let velocity = (G * self.main_planet.mass) / r.powi(2);

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
}
