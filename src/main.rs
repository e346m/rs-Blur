const RUST: u32 = 0xB7410E00;

fn main() {
    let r = red(RUST);
    let g = green(RUST);
    let b = blue(RUST);
    let a = alpha(RUST);
    println!("r: {}, g: {}, b: {}, a: {}", r, g, b, a);
}

fn red(color : u32) -> u32 {
    (color >> 24) & 0xff
}

fn green(color : u32) -> u32 {
    (color >> 16) & 0xff
}

fn blue(color : u32) -> u32 {
    (color >> 8) & 0xff
}

fn alpha(color : u32) -> u32 {
    (color >> 0) & 0xff
}
