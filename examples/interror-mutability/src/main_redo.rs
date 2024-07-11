//! Interior Mutability: Cell, RefCell, OnceCell
//! 
//! What is interior mutability
//!     - let a; let mut b;
//! 
//! Cell: https://doc.rust-lang.org/std/cell/struct.Cell.html

use std::cell::{Cell, OnceCell, RefCell};
use std::collections::HashMap;

#[derive(Default)]
struct SomeStruct {
    regular_field: u8,
    special_field: Cell<u8>
}

fn test_some_struct() {
    println!("Examples of Cell");
    let some_struct = SomeStruct::default();
    assert_eq!(some_struct.regular_field, 0);
    assert_eq!(some_struct.special_field.get(), 0);

    println!("change special_field of immutable some_struct");
    some_struct.special_field.set(100);
    assert_eq!(some_struct.regular_field, 0);
    assert_eq!(some_struct.special_field.get(), 100);
}

struct PhoneModel {
    brand: &'static str,
    model: &'static str,
    date_issue: &'static str,
    on_sale: Cell<bool> 
}

fn test_phone_model() {
    println!("A real example of phone model");
    let phone_model = PhoneModel { 
        brand: "ABC", model: "d100", date_issue: "2024-01-01",
        on_sale: Cell::new(true)
    };

    assert!(phone_model.on_sale.get());
    phone_model.on_sale.set(false);
    assert!(!phone_model.on_sale.get());
}

struct CarModel {
    brand: &'static str,
    model: &'static str,
    parameters: RefCell<HashMap<String, Vec<String>>>
}

fn test_car_model() {
    println!("Example of Car Model");
    let car_model = CarModel {
        brand: "XYZ", model: "m100",
        parameters: RefCell::new(HashMap::new())
    };

    assert!(car_model.parameters.borrow().is_empty());
    let mut parameters = car_model.parameters.borrow_mut();
    parameters.insert(
        "colors".to_string(),
        ["Blue", "Black", "Gray"].into_iter().map(String::from).collect()
    );
    drop(parameters);
    assert_eq!(car_model.parameters.borrow().get("colors").unwrap().len(), 3);
}

// borrow, borrow_mut
// borrow checking: runtime (Yes), compile (No)
fn ref_cell_borrow_checking() {
    println!("Example RefCell Borrow Checking");
    let car_model = CarModel {
        brand: "XYZ", model: "m100",
        parameters: RefCell::new(HashMap::new())
    };

    let _a = car_model.parameters.borrow();
    let _b = car_model.parameters.borrow();
    println!("multiple immutable borrows: OK");
    // let _c = car_model.parameters.borrow();
    // let _d = car_model.parameters.borrow_mut();
    // println!("immutable + mutable borrows: OK? No, runtime error, you will not see this line");
    // let _c = car_model.parameters.borrow_mut();
    // let _d = car_model.parameters.borrow_mut();
    // println!("mutable + mutable borrows: OK? No, runtime error, you will not see this line");
}

// OnceCell: lazy loading

struct LargeDataset {
    data: Vec<usize>
}

impl LargeDataset {
    fn new() -> Self {
        println!("initialize large dataset");
        Self { data: (0..1_000_000).collect() }
    }
}

fn compute(ds: &OnceCell<LargeDataset>, i: usize) {
    println!("computation for i {i}");
    let result = if i > 100 {
        ds.get_or_init(LargeDataset::new).data.iter().sum()
    } else {
        0
    };
    println!("computation for i {i} finished with result {result}");
}

fn test_larget_dataset() {
    let ds = OnceCell::new();
    for i in 99..103 {
        compute(&ds, i);
    }
}


fn main() {
    test_some_struct();
    test_phone_model();
    test_car_model();
    ref_cell_borrow_checking();
    test_larget_dataset();
}