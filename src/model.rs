use std::collections::{HashMap};
use std::borrow::Borrow;

pub struct Slot {
   pub plate: String,
   pub color: String
}


pub struct ParkingLot {
    pub capacity: i32,
    pub slots: HashMap<usize, Slot>
}

impl ParkMan for ParkingLot {
    fn allocate_slot(&mut self, car: Slot) -> usize {
        let free = self.get_first_available();
        if free == 0 {
            return 0
        }
        self.slots.insert(free, car);
        free
    }
    fn leave_slot(&mut self, slot: usize) -> usize {
        let res = self.slots.remove(&slot);
        return match res {
            Some(_) => {slot}
            None => {0}
        }
    }
    fn get_all_available_slots(&self) -> String {
        let mut table = String::from("Slot No     Plate    Color\n");
        for (key,val) in self.slots.iter() {
            table += format!("{}        {}        {}\n", key, val.plate, val.color).as_ref()
        }
        table
    }
    fn get_slots_by_color(&self, color: String) -> Vec<usize> {
        let mut res = Vec::new();
        for (key, val) in self.slots.iter() {
            if val.color == color {
                res.push(*key)
            }
        }
        res
    }
    fn slot_by_plate(&self, plate: String) -> usize {
    for (key, val) in self.slots.iter() {
        if val.plate == plate {
            return *key;
        }
    }
    return 0
    }

    fn get_plates_by_color(&self, color: String) -> Vec<String> {
        let mut plates: Vec<String> = vec![];
        for (key, val) in self.slots.iter() {
            if val.color == color {
                plates.push(String::from(&val.plate));
                plates.push(String::from(" "))
            }
        }
        if plates.len() == 0 {
            plates.push(String::from("Nothing"))
        }
        plates
    }

    fn get_first_available(&self) -> usize {
        for i in 1..self.capacity+1 {
            if !(self.slots.contains_key((i as usize).borrow())) {
                return i as usize
            }
        }
        0
    }

    fn create_parking_lot(&mut self, num: usize) -> usize{
        if self.capacity == 0 {
            self.capacity = num as i32;
            return num
        }
        0
    }
}

pub trait ParkMan {
    fn allocate_slot(&mut self, car: Slot) -> usize;
    fn leave_slot(&mut self, slot: usize) -> usize;
    fn get_all_available_slots(&self) -> String;
    fn get_slots_by_color(&self, color: String) -> Vec<usize>;
    fn slot_by_plate(&self, plate: String) -> usize;
    fn get_plates_by_color(&self, color: String) -> Vec<String>;
    fn get_first_available(&self) -> usize;
    fn create_parking_lot(&mut self, num: usize) -> usize;
}


