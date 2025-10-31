```
let v = Vector2D {
    x: Length::new::<meter>(10.0),
    y: Length::new::<meter>(10.0),
};
let p = Point {
    pos: Vector2D {
        x: Length::new::<meter>(0.0),
        y: Length::new::<meter>(0.0),
    },
    vel: Vector2D {
        x: Length::new::<meter>(1.0),
        y: Length::new::<meter>(0.0),
    },
    acc: Vector2D {
        x: Length::new::<meter>(0.0),
        y: Length::new::<meter>(0.0),
    },
    mass: Mass::new::<kilogram>(1.0),
};

println!(
    "Vector: x = {} m, y = {} m",
    v.x.get::<meter>(),
    v.y.get::<meter>()
);
println!("Magnitude: {} m", v.mag().get::<meter>());
println!(
    "Point: pos = ({}, {}), vel = ({}, {}), acc = ({}, {}), mass = {} kg",
    p.pos.x.get::<meter>(),
    p.pos.y.get::<meter>(),
    p.vel.x.get::<meter>(),
    p.vel.y.get::<meter>(),
    p.acc.x.get::<meter>(),
    p.acc.y.get::<meter>(),
    p.mass.get::<kilogram>()
);
println!();
```
