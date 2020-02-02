use crate::model::{ParkMan, Slot};
use std::cmp::Ordering;
use std::process::exit;


pub const CREATE: &str = "create_parking_lot";
pub const PARK: &str = "park";
pub const LEAVE: &str = "leave";
pub const STATUS: &str = "status";
pub const SLOTSBYCOLOR: &str = "get_slots_by_color";
pub const SLOTBYPLATE: &str = "get_slot_by_plate";
pub const PLATESBYCOLOR: &str = "get_plates_by_color";
pub const HELP: &str = "help";
pub const EXIT: &str = "exit";

pub fn process<T: ParkMan>(input: &mut Vec<&str>, p_lot: &mut T) -> String{

    match input.len() {
        1 => {
            match input[0] {
                STATUS => {
                    p_lot.get_all_available_slots()
                },
                HELP => {
                    String::from(format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                                         CREATE, PARK,LEAVE,STATUS, SLOTSBYCOLOR,
                                            SLOTBYPLATE, PLATESBYCOLOR))
                },
                EXIT => {exit(1)}
                _ => {String::from("unknown input\n Type `help` for available commands")}
            }
        },
        2 => {
            match input[0] {
                CREATE => {
                    let mut cap:i32 = 0;
                    let capacity = input[1].parse::<i32>();
                    match capacity {
                        Ok(_) => {cap = capacity.unwrap()},
                        Err(_) => {return String::from("only digits allowed")}
                    }
                    if cap <= 0 {
                        return String::from("capacity cannot be less or equal to 0")
                    }

                    let slot = p_lot.create_parking_lot(cap as usize);
                    if slot != 0{return String::from(format!("successfully created parking lot with {} slots", cap))}
                    String::from("parking lot has already been created")
                },
                LEAVE => {
                    let slot = input[1].parse::<i32>().expect("horrible value");
                    match slot.cmp(&0) {
                        Ordering::Less => {String::from("slot can not be less than 0")},
                        Ordering::Equal => {String::from("zero slot value not allowed")},
                        Ordering::Greater => {
                            let result = p_lot.leave_slot(slot as usize);
                            match result {
                                0 => {String::from(format!("slot {} is empty", slot))}
                                _ => {String::from(format!("successfully left slot {}", slot))}
                            }
                        }
                    }
                },
                SLOTBYPLATE => {
                    let plate = input[1];
                    let result = p_lot.slot_by_plate(String::from(plate));
                    if result == 0 {
                        return String::from(format!("no slot with plate {}", plate))
                    }
                    String::from(format!("your slot is {}", result))
                },
                SLOTSBYCOLOR => {
                    let color = input[1];
                    let mut answer = String::from(format!("slots with color {} are: \n", color));
                    let mut result = p_lot.get_slots_by_color(String::from(color));
                    if  result[0] == 0{
                         {return String::from("no found slots")}}
                        else { while let Some(slot) = result.pop() {
                            answer = answer + format!("{}\n", slot).as_ref();
                        }
                            answer
                        }
                },
                PLATESBYCOLOR => {
                    let no = String::from("Nothing");
                    let color = input[1];
                    let mut plates = p_lot.get_plates_by_color(String::from(color));
                    let mut result = String::from(format!("plates with color {} are\n", color));
                    if plates[0] == no {
                        {return String::from(format!("sorry, no plates with color {}", color))}}
                        else {
                            while let Some(plate) = plates.pop(){
                                result = result + plate.as_str();
                            }
                        }
                    result
                    },
                _ => {String::from("unknown inputs")}
            }

        },
        3 => {
            match input[0] {
                PARK => {
                    let new_slot = Slot{ plate: input[1].to_string(), color: input[2].to_string() };
                    let x = p_lot.allocate_slot(new_slot);
                    if x == 0 {
                        return String::from("parking lot is full");
                    }
                    String::from(format!("successfully parked you car to slot {}", x))
                },
                _ => {String::from("you're trying something i can't handle")}
            }
        }
        _ => {String::from("too many commands")}
    }
}

