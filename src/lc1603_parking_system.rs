struct ParkingSystem {
    slots_left: [i32; 4],
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        let slots_left = [0, big, medium, small];
        ParkingSystem {slots_left}
    }
    
    fn add_car(&mut self, car_type: i32) -> bool {
        if self.slots_left[car_type as usize] <= 0 {
            return false
        } else {
            self.slots_left[car_type as usize] -= 1;
            return true
        }
    }
}

/*
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */