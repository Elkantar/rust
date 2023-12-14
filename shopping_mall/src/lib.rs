pub mod mall;
pub use crate::mall::*;

// Importez `Mall` correctement
pub use crate::mall::floor::store;
pub use crate::mall::floor::store::employee::Employee;
pub use crate::mall::Mall;

pub fn biggest_store(mall: Mall) -> mall::floor::store::Store {
    let mut biggest_store: Option<mall::floor::store::Store> = None;

    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            if let Some(ref current_biggest) = biggest_store {
                if store.square_meters > current_biggest.square_meters {
                    biggest_store = Some(store.clone());
                }
            } else {
                biggest_store = Some(store.clone());
            }
        }
    }
    biggest_store.unwrap()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut highest_salary = 0.0;
    let mut highest_paid_employees = vec![];

    for floor in mall.floors.iter() {
        for store in floor.stores.iter() {
            for employee in store.employees.iter() {
                if (employee.salary - highest_salary).abs() < f64::EPSILON {
                    // If an employee has a salary close to the highest salary, add them to the list
                    highest_paid_employees.push(employee.clone());
                } else if employee.salary > highest_salary {
                    // If a new highest salary is found, clear the previous results
                    highest_salary = employee.salary;
                    highest_paid_employees.clear();
                    highest_paid_employees.push(employee.clone());
                }
            }
        }
    }

    // Debug print statements to inspect salary values
    println!("~~~Highest Salary: {:?}", highest_salary);
    println!(
        "~~~Employees with Highest Salary: {:#?}",
        highest_paid_employees
    );

    highest_paid_employees
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    let total_floor_size: u64 = mall
        .floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.square_meters)
        .sum();

    let total_guards = mall.guards.len();

    let total_employees: usize = mall
        .floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.employees.len())
        .sum();

    let total_all_employees = total_employees + total_guards;

    total_all_employees
}

pub fn check_for_securities(mall: &mut Mall, guards: Vec<guard::Guard>) {
    // Calculate the total floor size based on store square meters
    let total_floor_size: u64 = mall
        .floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .map(|store| store.square_meters)
        .sum();

    let current_guards = &mut mall.guards;

    let required_guards = (total_floor_size as f64 / 200.0).ceil() as usize;
    
    // Convert the reference to an integer and add them
    let total_required_guards = required_guards + current_guards.len();

    if total_required_guards > current_guards.len() {
        // Calculate how many additional guards we need
        let guards_to_add = total_required_guards - current_guards.len();

        // Create and add the required number of default guards
        for _ in 0..guards_to_add {
            current_guards.push(mall::guard::Guard::new("Default Guard", 25, 0));
        }
    }
}
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.iter_mut() {
        for store in floor.stores.iter_mut() {
            for employee in &mut store.employees {
                if employee.working_hours.1 - employee.working_hours.0 > 10 {
                    employee.raise(employee.salary * 0.10);
                } else {
                    employee.cut(employee.salary * 0.10);
                }
            }
        }
    }
}