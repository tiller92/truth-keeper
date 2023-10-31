// Goal to return all managers and there average salaries
use office::{Excel, Range, DataType};


pub fn manager_sal() {
    let mut excel = Excel::open("2023-Salaries-HealthSystem.xlsx").unwrap();
    let r = excel.worksheet_range("HealthSystem").unwrap();
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
}


