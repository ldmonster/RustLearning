// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Positions {
    MaintenanceCrew,
    MarketingDepartmentEmployees,
    Managers,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians,
}

enum Status {
    Active,
    Terminated,
}

struct Employee{
    position: Positions,
    status: Status,
}

impl Employee {
    fn enter_building(&self) -> Result<(), String>{
        match self.status {
            Status::Terminated => return Err("Employee status is terminated".to_string()),
            _ => (),
        }

        match self.position {
            Positions::MaintenanceCrew => Ok(()),
            Positions::MarketingDepartmentEmployees => Ok(()),
            Positions::Managers => Ok(()),
            _ => Err("this employee can't enter this building".to_string()),
        }
    }
}

fn get_employees() -> Vec<Employee>{
    vec![
        Employee{
            position: Positions::MaintenanceCrew,
            status: Status::Active,
        },
        Employee{
            position: Positions::MarketingDepartmentEmployees,
            status: Status::Terminated,
        },
        Employee{
            position: Positions::Managers,
            status: Status::Active,
        },
        Employee{
            position: Positions::LineSupervisors,
            status: Status::Terminated,
        },
        Employee{
            position: Positions::KitchenStaff,
            status: Status::Active,
        },
        Employee{
            position: Positions::AssemblyTechnicians,
            status: Status::Terminated,
        },
    ]
}

fn try_enter(employee: &Employee) -> Result<(), String>{
    let enter = employee.enter_building()?;
    println!("Trying to enter the building: OK");
    Ok(())
}

fn main() {
    let employee_vec = get_employees();
    for employee in employee_vec {
        match try_enter(&employee) {
            Err(e) => println!("Error: {:?}", e),
            _ => (),
        }

    }

}
