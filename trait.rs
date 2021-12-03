use std::fmt;
//特征
pub trait GetInformation {
    fn get_name(&self) -> &String;
    fn get_age(&self) -> u32;
}

pub struct Student {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GetInformation for Student {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

pub struct Teacher {
    pub name: String,
    pub age: u32,
    pub subject: String,
}

impl GetInformation for Teacher {
    fn get_name(&self) -> &String {
        &self.name
    }
    fn get_age(&self) -> u32 {
        self.age
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
fn print_information(item: impl GetInformation) {
    println!("name={}", item.get_name());
    println!("name={}", item.get_age());
}

fn main() {
    let s = Student {
        name: "X".to_string(),
        age: 200,
        subject: "zongshuji".to_string(),
    };
    let t = Teacher {
        name: "J".to_string(),
        age: 1000,
        subject: "zongshuji".to_string(),
    };
    //print_information(s);
    println!("Student:");
    print_information(s);
    println!("Teacher:");
    print_information(t);

    //println!("student name={},age={}", s.get_name(),s.get_age());
    //println!("teacher name={},age={}",t.get_name(),t.get_age());
}
