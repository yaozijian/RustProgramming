
struct Car{
    color:i32,
    weight:i32,
}

impl Car{

    fn show(&self){
        println!("Car color={} weight={}",self.color,self.weight);
    }

    fn showcar(car:Car){
        car.show();
    }
}

impl Clone for Car{
    fn clone(&self) -> Self{
        return Car{
            color: self.color + 10,
            weight: self.weight + 10,
        }
    }
}

impl Drop for Car{
    fn drop(&mut self){
        println!("car dropped: color={} weight={}",self.color,self.weight);
    }
}

pub fn ownership() {
    let car = Car{
        color: 1,
        weight: 2,
    };
    Car::showcar(car.clone());
    car.show();
}
