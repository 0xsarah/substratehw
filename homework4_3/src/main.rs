use std::io;

//create 3 structs represent shapes
struct Circle2D {
    r: f32,
}

struct Triangle2D {
    w: f32,
    h: f32,
}
 
struct Square2D {
    w: f32,
    h: f32,
}

//write 3 fucntions to calculates the surface area
fn circleArea(dim: &mut Circle2D) -> f32 {
    3.14 * dim.r*dim.r
}

fn squareArea(dim: &mut Square2D) -> f32 {
    dim.w * dim.h
}

fn triangleArea(dim: &mut Triangle2D) -> f32 {
    dim.w * dim.h * 0.5
}

//in main, takes 2 input: shape, parameters

fn main() {
    let mut input = String::new();
    println!("Please select the shape type:");
    println!("Triangle");
    println!("Square");
    println!("Circle");
    
    io::stdin()
        .read_line(&mut input)
        .expect("Fail to understand");

    println!("The shape is: {}",input);
    if input.trim() == "Triangle"{
        println!("please enter the width:");
        let mut width = String::new();
        io::stdin()
        .read_line(&mut width)
        .expect("Fail to understand");
        let width:f32 = width.trim().parse().expect("Please type a number!");


        println!("please enter the height:");
        let mut height = String::new();
        io::stdin()
        .read_line(&mut height)
        .expect("Fail to understand");  
        let height:f32 = height.trim().parse().expect("Please type a number!");

        let mut surface = Triangle2D{w :width, h :height};
        let ans= triangleArea(&mut surface);
        
        println!("The surface area is {}", ans);

    }else if input.trim() == "Square"{
        println!("please enter the width:");
        let mut width = String::new();
        io::stdin()
        .read_line(&mut width)
        .expect("Fail to understand");
        let width:f32 = width.trim().parse().expect("Please type a number!");


        println!("please enter the height:");
        let mut height = String::new();
        io::stdin()
        .read_line(&mut height)
        .expect("Fail to understand");  
        let height:f32 = height.trim().parse().expect("Please type a number!");

        let mut surface = Square2D{w :width, h :height};
        let ans= squareArea(&mut surface);
        
        println!("The surface area is {}", ans);

    }else if input.trim() == "Circle"{
        println!("please enter the radius:");
        let mut radius = String::new();
        io::stdin()
        .read_line(&mut radius)
        .expect("Fail to understand");
        let radius:f32 = radius.trim().parse().expect("Please type a number!");


        let mut surface = Circle2D{r :radius};
        let ans= circleArea(&mut surface);
        
        println!("The surface area is {}", ans);

    }else{
        println!("Fail to understand");
    }     
}