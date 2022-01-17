use std::{thread, time::Duration, vec};

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("시간이 오래 걸리는 계산을 수행중...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "오늘은 {}번의 팔굽혀펴기를 하세요!",
            expensive_result.value(intensity)
        );
        println!(
            "다음은 {}번의 윗몸일으키기를 하세요!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!(
                "오늘은 {}분간 달리기를 하세요!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specificated_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specificated_value, simulated_random_number);

    let x = 4;

    let equal_to_x = move |z| z == x;

    let y = 4;

    // 클로저에서 move 키워드를 통해 소유권이 이동하여 여기서 x변수를 호출할 수 없다
    // println!("변수 x를 사용할 수 없습니다: {:?}", x);
    // let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
