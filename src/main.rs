use std::{env, process, thread, time::Duration};

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

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("today, do {} pushups!", expensive_closure.value(intensity));
        println!("next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today! remember to stay hydrated!");
        } else {
            println!(
                "today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn call_with_different_values() {
        let mut c = super::Cacher::new(|a| a);
        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
