use office::{Excel,DataType};
// should have a seprate fn that opens a docs or docs
//First goal is to just count bedside vs non-bedside
pub fn managers(path: &str){
    let mut excel = Excel::open("/Users/ryantiller/Documents/rust_projects/rust-excel/excelfilter/osu-salaris/earning-2022/2022-Earnings-HealthSystem.xlsx").unwrap();
    let health_system = excel.worksheet_range("HealthSystem").unwrap();
    let mut manager_list: Vec<DataType> = Vec::new(); 
    let mut managers_gross_salaries: Vec<DataType> = Vec::new();
    let mut clinical_gross_salaries: Vec<DataType> = Vec::new();
    let mut clinical: Vec<DataType> = Vec::new();
    let mut clinical_gross_salaris: Vec<DataType> = Vec::new();

    println!("{:?}", health_system.get_size());
    //.skip(1).take(1) add to skip fors and cap
        for row in health_system.rows().skip(1) {
            for cell in row.iter().skip(2).take(5) {
                match cell {
                    DataType::Empty => print!("empty /t"),
                    DataType::String(s) => {
                       if manager_supervisor_director(s) {
                            manager_list.push(row[2].clone());
                            managers_gross_salaries.push(row[10].clone());
                       }else{
                            clinical_gross_salaries.push(row[10].clone());
                       };
                },
                    DataType::Float(f) => println!("{:?}",f),
                    DataType::Int(i) => print!("{:?} \t",i),
                    DataType::Bool(b) => print!("{:?} \t",b),
                    DataType::Error(e) => print!("Error: {:?}\t", e),
                }
            }
        }
    println!("{:?}",&managers_gross_salaries);
    match  avgerage_sal(&managers_gross_salaries){
        Some(avg) => println!("The average salarie for a supervisor director and manager is {:?}", avg),
        None => println!("Error at match funtion in manager function"),
    }
    println!("managers = {:?}", manager_list.len());
    println!("clinical = {:?}", clinical.len());
}



pub fn manager_supervisor_director(role: &str) ->bool {
// takes a strign check if it is clinical or non-clinical role returns true or false
    let manager = String::from("Manager");
    let supervisor = String::from("Supervisor");
    let director = String::from("Director");
    let executive = String::from("Executive");
 
        if role.contains(&manager){
            true
        }else if role.contains(&supervisor){
            true
        }else if role.contains(&director){
            true
        }else if role.contains(&executive){
            true
        }else{
            false
        }
}

pub fn avgerage_sal(arr: &Vec<DataType>) -> Option<f64> {
    println!("arr: {:?}",arr);
    if arr.is_empty() {
        return None;
    };
  
    let mut sum = 0.0;
    for i in arr.iter() {
        match i {
            DataType::Empty => print!("empty /t"),
            DataType::String(s) => println!("{:?}",s),
            DataType::Float(f) => sum += f,
            DataType::Int(i) => print!("{:?} \t",i),
            DataType::Bool(b) => print!("{:?} \t",b),
            DataType::Error(e) => print!("Error: {:?}\t", e),
        }
    }
    let num = arr.len();
    
    let avg: f64 = sum / num as f64;
    Some(avg)
}