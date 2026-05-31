fn main() {
    println!("Hello, world & GitHub!");

    let a = Pos{x: 1.0, y: 5.0};
    let b = Pos{x: 5.0, y: -3.0};
    
    let c = a.add(b);

    println!("{a:?} + {b:?} = {c:?}");
}

#[derive(Debug, Clone, Copy)]
struct Pos{
    x: f32,
    y: f32,
}

impl Pos{
    fn add(&self, other: Self) -> Self{
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
