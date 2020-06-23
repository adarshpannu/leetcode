// Ref: http://rosettacode.org/wiki/100_doors

pub fn run() {
    let mut doors: [bool; 101] = [false; 101];

    for pass in 1..=100 {
        let mut door_num = pass;
        while door_num < 100 {
            doors[door_num] = !doors[door_num];
            door_num += pass;
        }
    }
    for door_num in 1..=100 {
        if doors[door_num] {
            println!("Door {} is open", door_num);
        }
    }
}

