use office::{Excel,DataType};


//only have one year of data so just filter mangers info
pub fn manager_sal(path: &str){
    let mut excel = Excel::open(path).unwrap();
    let health_system = excel.worksheet_range("HealthSystem").unwrap();
    let mut non_clinical: Vec<String>;
    println!("{:?}", health_system.get_size());
        for row in health_system.rows().skip(1).take(2) {
            for cell in row {
                // Print the value of each cell  
                match cell {
                    DataType::Empty => print!("Empty\t"),
                    DataType::String(s) => print!(" {}\t", s),
                    DataType::Float(f) => print!(" {}\t", f),
                    DataType::Int(i) => print!(" {}\t", i),
                    DataType::Bool(b) => print!(" {}\t", b),
                    DataType::Error(e) => print!("Error: {:?}\t", e),
                    _ => print!("Other\t"),
                }
            }
            // println!();// Move to the next line after each row
        }
        }



