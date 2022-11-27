pub mod biggest_number {
    pub fn biggest_number(number_list: &[i32]) -> i32 {
        let mut largest = number_list[0];
        for &item in number_list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
}

pub mod additon {
    pub struct Numbers {
        a: i32,
        b: i32,
    }

    impl Numbers {
        pub(crate) fn new(a: i32, b: i32) -> Numbers {
            Numbers { a, b }
        }

        pub(crate) fn add(&self) -> i32 {
            self.a + self.b
        }

        pub(crate) fn sub(&self) -> i32 {
            self.a - self.b
        }
    }
}
