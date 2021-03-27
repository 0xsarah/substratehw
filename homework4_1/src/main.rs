fn main() {
    let light = TrafficLight::Green;
    
    println!("light time is: {}", light.time());
}

enum TrafficLight {
    Red,
    Green,
    Yellow,
    Rainbow,
}

impl TrafficLight{
        fn time(&self) -> u8 {
           match self {
               TrafficLight::Red => 150,
               TrafficLight::Green => 60,
               TrafficLight::Yellow => 5,
               _ => 0,
           }

   
        }
}

