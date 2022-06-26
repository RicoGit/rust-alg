//! 1603. Design Parking System

struct ParkingSystem {
    slots: Vec<usize>
}

impl ParkingSystem {

    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            slots: vec![big as usize, medium as usize, small as usize]
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.slots[car_type as usize - 1] > 0 {
            self.slots[car_type as usize - 1] -= 1;
            true
        } else {
            false
        }
    }
}

struct Solution;
