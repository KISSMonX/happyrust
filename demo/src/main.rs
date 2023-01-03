fn main() {
    let w = 233;
    let l = 10;

    let rect = (30, 50);

    let rect2 = Rectangle {
        width: 100,
        length: 100,
    };

    println!("{}", area(w, l));
    println!("{}", area2(rect));
    println!("{}", area3(&rect2));
    println!("{}", area4(rect2));
}


struct Rectangle {
    width: u32,
    length: u32,
}

fn area4(rect: Rectangle) -> u32 {
    rect.width * rect.length
}


fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}


fn area2(dim: (u32, u32)) -> u32 {
    dim.0*dim.1
}


fn area(width: u32, length: u32) ->u32 {
    return width *length;
}