pub use mall::{*, floor::store::employee::Employee, guard::Guard};
pub use mall::floor::*;

pub mod mall;

//Using the mall module provided, create the following functions to help run a shopping mall:
//
// biggest_store: receives a Mall and returns the Store with the biggest square_meters.
pub fn biggest_store(galleria: Mall) -> store::Store {
    galleria
        .floors
        .iter()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|store| store.square_meters)
        .unwrap()
        .clone()
}

// highest_paid_employee: receives a Mall and returns a vector containing the Employee(s) with the highest salary.
pub fn highest_paid_employee(galleria: Mall) -> Vec<Employee> {
    let mut employees: Vec<&Employee> = galleria
        .floors
        .iter()
        .flat_map(|floor| floor.stores.iter().flat_map(|store| store.employees.iter()))
        .collect();
    employees.sort_by(|a, b| b.salary.partial_cmp(&a.salary).unwrap());
    let highest_paid_salary = employees.first().map_or(0.0, |e| e.salary);
    let result: Vec<Employee> = employees
        .into_iter()
        .filter(|e| e.salary == highest_paid_salary)
        .cloned()
        .collect();
    result
        .iter()
        .map(|elem| Employee {
            salary: format!("{:.4}", elem.salary).parse::<f64>().unwrap_or(0.0),
            ..elem.clone()
        })
        .collect()
}

// nbr_of_employees: receives a Mall and returns the number of employees and guards as a usize.
pub fn nbr_of_employees(galleria: Mall) -> usize {
    galleria
        .floors
        .iter()
        .flat_map(|floor| floor.stores.iter().flat_map(|store| store.employees.iter()))
        .count()
        + galleria.guards.len()
}

// check_for_securities: receives a Mall and a vector of Guard. If there is not at least 1 guard for every 200 square meters of floor size, a guard should be added to the Mall.guards.
pub fn check_for_securities(galleria: &mut Mall, guards: Vec<Guard>) {
    let total_floor_size: u64 = galleria.floors.iter().map(|f| f.size_limit).sum();
    let required_guards = (total_floor_size / 200) as usize - galleria.guards.len();
    for i in 0..required_guards {
        galleria.hire_guard(guards[i].clone())
    }
}

// cut_or_raise: receives a Mall.
// For each employee, the salary will be raised by 10% if they work more than 10 hours, else their salary will be decreased by 10%. You can consider that guards are not employees of the mall.
pub fn cut_or_raise(galleria: &mut Mall) {
    for floor in galleria.floors.iter_mut() {
        for store in floor.stores.iter_mut() {
            for employee in store.employees.iter_mut() {
                if employee.working_hours.1 - employee.working_hours.0 > 10 {
                    employee.salary *= 1.1;
                } else {
                    employee.salary *= 0.9;
                }
                employee.salary = format!("{:.4}", employee.salary)
                    .parse::<f64>()
                    .unwrap_or(0.0);
            }
        }
    }
}