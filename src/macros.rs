#[macro_export]
macro_rules! vec2 {
    ($pair: expr) => {
        Vector2 {x:($pair).0 as f32, y:($pair).1 as f32}
    };

    ($x:expr, $y:expr) => {
        Vector2 {x:($x) as f32, y:($y) as f32}
    };
}
