#[derive(Debug)]
enum EmployeeType {
    JuniorEngineer,
    SeniorEngineer,
}
#[derive(Debug)]
struct Employee{
    name:String,
    salary:u32,
    id:String,
    employee_type:EmployeeType

}
impl Employee {
    fn new(name: String, id: String, employee_type: EmployeeType) -> Employee {
        let salary = match employee_type {
            EmployeeType::JuniorEngineer => 50000,
            EmployeeType::SeniorEngineer => 60000,
        };

        Employee {
            name,
            salary,
            id,
            employee_type,
        }
    }
}
fn main() {
    let emp1 = Employee::new(String::from("Alice"), "1".to_string(), EmployeeType::JuniorEngineer);
    let emp2 = Employee::new(String::from("Bob"), "2".to_string(), EmployeeType::SeniorEngineer);

    println!("{:?}", emp1);
    println!("{:?}", emp2);
}
