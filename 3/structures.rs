#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Estrutura unitária
struct Unit;

//Tuple
struct Pair(i32, f32);

// Estrutura do tipo (x, y)
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Estrutura que recebe outra estrutura.
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32{
    let Point{x: x1, y: y1} = rect.top_left;
    let Point{x: x2, y: y2} = rect.bottom_right;

    let area:f32 = (x1 - x2)*(y1 - y2);
    return area
}

fn square(point: Point, proportion:f32) -> Rectangle{
    let bottom_right: Point = 
        Point {x: point.x + proportion, 
               y: point.y + proportion};
    return Rectangle{
        top_left: point,
        bottom_right: bottom_right,
    }
}

fn main() {
    let name: String = String::from("Dostoievski");
    let age = 123;
    let dostoievski = Person {name, age};
    println!("{:?}", dostoievski);

    let point: Point = Point {x: 132.1, y: 0.323};
    println!("Point coordinates: ({}, {})", point.x, point.y);
    let bottom_right = Point { x: 43.54, ..point};
    println!("Point coordinates: ({}, {})", bottom_right.x
                                          , bottom_right.y);

    // Destructure de point.
    let Point {x: left_edge, y: _top_edge} = point;
    let rectangle = Rectangle{
        top_left: Point{x: left_edge, y: 0.456},
        bottom_right: bottom_right,
    };

    // Instância de unit.
    let _unit = Unit;

    let pair = Pair(1, 0.1);
    // Acesso da estrutura da tupla
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Outro Destructure da tupla
    let Pair(int, decimal) = pair;

    println!("pair contains {:?} and {:?}", int, decimal);

    let area = rect_area(rectangle);
    println!("{}", area);

    println!("{:?}", square(point, 3.1));
}