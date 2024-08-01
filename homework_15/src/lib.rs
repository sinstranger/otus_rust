pub enum Item {
    First,
    Second,
    Third,
}

impl Item {
    pub fn index(&self) -> usize {
        match self {
            Item::First => 0,
            Item::Second => 1,
            Item::Third => 2,
        }
    }
}

pub trait ItemSet {
    fn default_values() -> Self;

    fn get_item(&self, item: Item) -> f64;

    fn set_item(&mut self, item: Item, value: f64);

    fn is_default(&self) -> bool {
        self.sum() == 0.0
    }

    fn sum(&self) -> f64 {
        let first = self.get_item(Item::First) as f64;
        let second = self.get_item(Item::Second) as f64;
        let third = self.get_item(Item::Third) as f64;
        first + second + third
    }
}

#[derive(Debug)]
pub struct Tuple(u32, f32, f64);

impl ItemSet for Tuple {
    fn default_values() -> Self {
        Self(0, 0.0, 0.0)
    }

    fn get_item(&self, item: Item) -> f64 {
        match item {
            Item::First => self.0 as _,
            Item::Second => self.1 as _,
            Item::Third => self.2,
        }
    }

    fn set_item(&mut self, item: Item, value: f64) {
        match item {
            Item::First => self.0 = value as _,
            Item::Second => self.1 = value as _,
            Item::Third => self.2 = value,
        };
    }
}

#[derive(Debug)]
pub struct Array([f64; 3]);

impl ItemSet for Array {
    fn default_values() -> Self {
        Self([0.0; 3])
    }

    fn get_item(&self, item: Item) -> f64 {
        self.0[item.index()]
    }

    fn set_item(&mut self, item: Item, value: f64) {
        self.0[item.index()] = value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_sum<T: ItemSet>() {
        let mut instance = T::default_values();
        perform_set_operations(&mut instance, 1.0, 2.0, 3.0);
        assert_eq!(instance.sum(), 6.0)
    }

    fn test_default<T: ItemSet>() {
        let instance = T::default_values();
        assert!(instance.is_default());
        assert_eq!(instance.sum(), 0.0)
    }

    fn perform_set_operations<T: ItemSet>(t: &mut T, first: f64, second: f64, third: f64) {
        t.set_item(Item::First, first);
        t.set_item(Item::Second, second);
        t.set_item(Item::Third, third);
    }

    #[test]
    fn test_tuple_defaul() {
        test_default::<Tuple>();
    }

    #[test]
    fn test_tuple_sum() {
        test_sum::<Tuple>();
    }

    #[test]
    fn test_tuple_operations() {
        let mut x = Tuple::default_values();
        perform_set_operations(&mut x, 1.0, 2.0, 3.0);
        assert_eq!(x.get_item(Item::First), 1.0);
        assert_eq!(x.get_item(Item::Second), 2.0);
        assert_eq!(x.get_item(Item::Third), 3.0);
    }

    #[test]
    fn test_array_defaul() {
        test_default::<Array>()
    }

    #[test]
    fn test_array_sum() {
        test_sum::<Array>();
    }

    #[test]
    fn test_array_operations() {
        let mut x = Array::default_values();
        perform_set_operations(&mut x, 1.0, 2.0, 3.0);
        assert_eq!(x.get_item(Item::First), 1.0);
        assert_eq!(x.get_item(Item::Second), 2.0);
        assert_eq!(x.get_item(Item::Third), 3.0);
    }
}
