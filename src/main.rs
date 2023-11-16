enum TrafficLight {
    Red,
    Yellow,
    Green,
}

trait Time {
    fn time(&self) -> u8;
}

impl Time for TrafficLight {
    fn time(&self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 30,
        }
    }
}

fn main() {
    let red = TrafficLight::Red;
    let yellow = TrafficLight::Yellow;
    let green = TrafficLight::Green;
    
    println!("Red light time: {} seconds", red.time());
    println!("Yellow light time: {} seconds", yellow.time());
    println!("Green light time: {} seconds", green.time());
}
