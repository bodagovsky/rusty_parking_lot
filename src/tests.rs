
#[cfg(test)]
mod tests {
    use super::super::model::{Slot, ParkingLot, ParkMan};
    use std::collections::HashMap;

    #[test]
    fn test_leave_slot() {
        let plate = String::from("HU9-9-IU");
        let color = String::from("White");
        let slot = Slot{
            plate,
            color
        };
        let mut slots:HashMap<usize, Slot> = Default::default();
        slots.insert(1,slot);
        let mut park = ParkingLot{
            capacity: 5,
            slots

        };
        let fail = park.leave_slot(5);
        let success = park.leave_slot(1);
        assert_eq!(fail, 0);
        assert_eq!(success, 1)
    }

    #[test]
    fn test_get_slots_by_color() {
        let slot = Slot{
            plate: String::from("Hello"),
            color: String::from("Black")
        };
        let mut slots:HashMap<usize, Slot> = Default::default();
        slots.insert(1,slot);
        let mut park = ParkingLot{
            capacity: 5,
            slots

        };
        let black = park.get_slots_by_color(String::from("Black"));
        assert_eq!(black[0], 1 as usize)

    }

    #[test]
    fn test_slot_by_plate() {
        let slot = Slot{
            plate: String::from("HH-UY-90"),
            color: String::from("Black")
        };
        let mut slots:HashMap<usize, Slot> = Default::default();
        slots.insert(1,slot);
        let mut park = ParkingLot{
            capacity: 5,
            slots

        };
        let desired_slot = park.slot_by_plate(String::from("RANDOM_PLATE"));
        assert_eq!(desired_slot, 0);
        let desired_slot = park.slot_by_plate(String::from("HH-UY-90"));
        assert_eq!(desired_slot, 1)
    }

    #[test]
    fn test_plates_by_color() {
        let slot = Slot{
            plate: String::from("HH-UY-90"),
            color: String::from("Black")
        };
        let mut slots:HashMap<usize, Slot> = Default::default();
        slots.insert(1,slot);
        let mut park = ParkingLot{
            capacity: 5,
            slots

        };

        let plates = park.get_plates_by_color(String::from("Black"));
        assert_eq!(plates[0], String::from("HH-UY-90"));
        let empty = park.get_plates_by_color(String::from("Red"));
        assert_eq!(empty[0], String::from("Nothing"));
    }

    #[test]
    fn test_all_slots() {
        let slot = Slot{
            plate: String::from("HH-UY-90"),
            color: String::from("Black")
        };
        let slot2 = Slot{
            plate: String::from("HH-UY-10"),
            color: String::from("Red")
        };
        let slot3 = Slot{
            plate: String::from("MM-IO-09"),
            color: String::from("White")
        };
        let mut slots:HashMap<usize, Slot> = Default::default();
        slots.insert(1,slot);
        slots.insert(2,slot2);
        slots.insert(3,slot3);
        let mut park = ParkingLot{
            capacity: 5,
            slots

        };
        let result = park.get_all_available_slots();
        print!("{}",result)
    }

    #[test]
    fn test_first_available() {
        let slot = Slot{
            plate: String::from("HH-UY-90"),
            color: String::from("Black")
        };
        let slot2 = Slot{
            plate: String::from("HH-UY-10"),
            color: String::from("Red")
        };
        let slot3 = Slot{
            plate: String::from("MM-IO-09"),
            color: String::from("White")
        };
        let mut slots:HashMap<usize, Slot> = Default::default();
        slots.insert(1,slot);
        slots.insert(2,slot2);
        slots.insert(3,slot3);
        let mut park = ParkingLot{
            capacity: 5,
            slots

        };
        park.leave_slot(2);
        let x = park.get_first_available();
        assert_eq!(x, 2);
        let new_slot =  Slot{
            plate: String::from("GG-YY-09"),
            color: String::from("Blue")
        };
        park.slots.insert(2,new_slot);
        let y = park.get_first_available();
        assert_eq!(y, 4)
    }

    #[test]
    fn test_allocate_slot() {
        let slot = Slot{
            plate: String::from("HH-UY-90"),
            color: String::from("Black")
        };
        let slot2 = Slot{
            plate: String::from("HH-UY-10"),
            color: String::from("Red")
        };
        let slot3 = Slot{
            plate: String::from("MM-IO-09"),
            color: String::from("White")
        };
        let mut slots:HashMap<usize, Slot> = Default::default();
        slots.insert(1,slot);
        slots.insert(2,slot2);
        slots.insert(3,slot3);
        let mut park = ParkingLot{
            capacity: 5,
            slots

        };
        park.leave_slot(2);
        let x = park.allocate_slot(Slot{
            plate: String::from("plate"),
            color: String::from("color")
        });
        assert_eq!(x, 2);
        let y = park.allocate_slot(Slot{
            plate: String::from("plate2"),
            color: String::from("color2")
        });
        assert_eq!(y, 4);

        let z = park.allocate_slot(Slot{
            plate: String::from("plate3"),
            color: String::from("color3")
        });
        assert_eq!(z, 5);

        let q = park.allocate_slot(Slot{
            plate: String::from("plate4"),
            color: String::from("color4")
        });
        assert_eq!(q, 0)
    }

}