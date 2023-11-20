use office::{Excel,DataType};


//First goal is to just count bedside vs non-bedside
pub fn managers(path: &str){
    let mut excel = Excel::open("/Users/ryantiller/Documents/rust_projects/rust-excel/excelfilter/osu-salaris/earning-2022/2022-Earnings-HealthSystem.xlsx").unwrap();
    let health_system = excel.worksheet_range("HealthSystem").unwrap();
    let mut mangers: Vec<DataType> = Vec::new(); 
    let mut clinical: Vec<DataType> = Vec::new();

    println!("{:?}", health_system.get_size());
    //.skip(1).take(1) add to skip fors and cap
        for row in health_system.rows().skip(1).take(2) {
            for cell in row.iter().skip(2).take(1) {
                println!("{:?}", row);
                match cell {
                    DataType::Empty => print!("empty /t"),
                    DataType::String(s) => {
                       if manager_supervisor_director(s) {
                            mangers.push(row[10].clone());
                       }else{
                            clinical.push(row[10].clone());
                       };
                },
                    DataType::Float(f) => print!("{:?} \t",f),
                    DataType::Int(i) => print!("{:?} \t",i),
                    DataType::Bool(b) => print!("{:?} \t",b),
                    DataType::Error(e) => print!("Error: {:?}\t", e),
                }
            }
        }
        println!("manager vec = {:?}", mangers);
        println!("clinical vec = {:?}", clinical);
        }


pub fn manager_supervisor_director(role: &str) ->bool {
// takes a strign check if it is clinical or non-clinical role returns true or false
    let manager = String::from("Manager");
    let supervisor = String::from("Supervisor");
    let director = String::from("Director");
 
        if role.contains(&manager){
            println!("non-clinical");
            true
        }else if role.contains(&supervisor){
            true
        }else if role.contains(&director){
            true
        }else{
            false
        }
}

