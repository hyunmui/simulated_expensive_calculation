use std::{thread, time::Duration};

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("시간이 오래 걸리는 계산을 수행중...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요!",
            expensive_closure(intensity)
        );
        println!(
            "다음은 {}번의 윗몸일으키기를 하세요!",
            expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                expensive_closure(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specificated_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specificated_value, simulated_random_number);
}
