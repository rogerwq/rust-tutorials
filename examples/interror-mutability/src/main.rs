//! Interior Mutability
//! 
//! immutable, mutable
//! let x; let mut y;
//! 
//! Cell, RefCell, OnceCell

use std::{cell::{Cell, OnceCell, RefCell}, collections::HashMap};

#[derive(Default)]
struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>,
}

struct PhoneModel {
    brand: &'static str,
    model: &'static str,
    date_issued: &'static str,
    on_sale: Cell<bool>, // false, true, false
    // cell_try: Cell<HashMap<String, String>>
}

#[derive(Debug)]
struct CarModel {
    brand: &'static str,
    model: &'static str,
    parameters: RefCell<HashMap<String, Vec<String>>>
}

struct LargeDataset {
    data: Vec<usize>
}

impl LargeDataset {
    fn new() -> Self {
        println!("initializing the large dataset");
        Self { data: (0..1_000_000).collect() }
    }
}

fn compute(i: usize, dataset: &OnceCell<LargeDataset>) {
    println!("calculating for i {i}");
    let res = if i > 100 { 
        dataset.get_or_init(LargeDataset::new).data.iter().sum()
    } else { 0 };
    println!("calculated for i {i} with result {res}");
}

fn main() {
    println!("An example of Cell");
    let some_struct = SomeStruct::default();
    assert!(some_struct.regular_field == 0);
    assert!(some_struct.special_field.get() == 0);
    // Error: some_struct.regular_field = 1;
    some_struct.special_field.set(100);
    assert!(some_struct.special_field.get() == 100);

    println!("Another example of Cell");
    let phone = PhoneModel { 
        brand: "ABC", model: "d100", date_issued: "2024-01-01", 
        on_sale: Cell::new(false),
        // cell_try: Cell::new(HashMap::new())
    };
    assert!(!phone.on_sale.get());
    phone.on_sale.set(true);
    assert!(phone.on_sale.get());
    // dbg!(phone.cell_try.get());

    println!("An example of RefCell");
    let car = CarModel {
        brand: "XYZ", model: "w100",
        parameters: RefCell::new([("colors".to_string(), vec!["red".to_string()])].into_iter().collect())
    };
    let mut parameters = car.parameters.borrow_mut();
    parameters.get_mut("colors").unwrap().push("black".to_string());
    drop(parameters);
    // RefCell: borrow, borrow_mut
    // borrow checking: compile (X), runtime (O)
    // let _a = car.parameters.borrow();
    // let _b = car.parameters.borrow();
    // let _a = car.parameters.borrow_mut();
    // let _b = car.parameters.borrow();
    // let _a = car.parameters.borrow_mut();
    // let _b = car.parameters.borrow_mut();

    println!("An example of OnceCell");
    // OnceCell: lazy initialization
    // let large_ds = LargeDataset::new();
    let large_ds = OnceCell::new();
    compute(0, &large_ds);
    compute(1, &large_ds);
    compute(99, &large_ds);
    compute(100, &large_ds);
    compute(101, &large_ds);
    compute(102, &large_ds);
    compute(103, &large_ds);
}
