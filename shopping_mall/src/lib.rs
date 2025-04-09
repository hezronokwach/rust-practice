pub mod mall;
pub use guard::Guard;
pub use mall::floor::store::employee::Employee;
pub use mall::floor::store::*;
pub use mall::floor::*;
pub use mall::*;
/// Returns the store with the largest square meters in the mall
pub fn biggest_store(mall: Mall) -> Store {
    mall.floors
        .iter()
        .flat_map(|floor| &floor.stores)
        .max_by_key(|store| store.square_meters)
        .cloned()
        .unwrap()
}

/// Returns a vector of employees with the highest salary in the mall
pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    let mut highest_salary = 0.0;
    let mut highest_paid: Vec<Employee> = Vec::new();
    
    // First pass: find the highest salary
    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                }
            }
        }
    }
    
    // Second pass: collect all employees with that salary
    for floor in &mall.floors {
        for store in &floor.stores {
            for employee in &store.employees {
                if employee.salary == highest_salary {
                    highest_paid.push(employee.clone());
                }
            }
        }
    }
    
    highest_paid
}

/// Returns the total number of employees and guards in the mall
pub fn nbr_of_employees(mall: Mall) -> usize {
    let mut count = mall.guards.len();
    
    for floor in &mall.floors {
        for store in &floor.stores {
            count += store.employees.len();
        }
    }
    
    count
}

/// Ensures there is at least 1 guard for every 200 square meters of floor size
/// If not, adds guards from the provided vector to meet the requirement
pub fn check_for_securities(mall: &mut Mall, mut guards: Vec<Guard>) {
    let mut total_size = 0;
    
    // Calculate total mall size
    for floor in &mall.floors {
        for store in &floor.stores {
            total_size += store.square_meters;
        }
    }
    
    // Calculate required number of guards (1 per 200 square meters)
    let required_guards = (total_size + 199) / 200; // Ceiling division
    
    // Add guards if needed
    while mall.guards.len() < required_guards as usize && !guards.is_empty() {
        if let Some(guard) = guards.pop() {
            mall.hire_guard(guard);
        }
    }
}

/// Adjusts employee salaries: +10% if they work more than 10 hours, -10% otherwise
pub fn cut_or_raise(mall: &mut Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for employee in &mut store.employees {
                let (entry, exit) = employee.working_hours;
                let hours_worked = if exit > entry {
                    exit - entry
                } else {
                    24 - entry + exit // Handle cases like 22:00 to 6:00
                };
                
                if hours_worked > 10 {
                    // Raise salary by 10%
                    employee.salary *= 1.1;
                } else {
                    // Cut salary by 10%
                    employee.salary *= 0.9;
                }
            }
        }
    }
}
