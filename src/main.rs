#[allow(dead_code)]
#[derive(Debug)]
#[allow(unused_variables)]
// enum StudentSubSection{

//     ClassCaptain,
//     Prefect,
// }
// enum StudentPosition {
//     Position(Option<StudentSubSection>),
//     // ClassCaptain,
//     // Prefect,

// }
enum StudentPosition {
    ClassCaptain,
    Prefect,
    None,
    
}

#[allow(dead_code)]
#[derive(Debug)]
#[allow(unused_variables)]
enum Position {
    Student { grade: Option<u8>,student_position:StudentPosition },
    Teacher { salary: f64 },
    Principal,
    VicePrinciple,
}

#[allow(dead_code)]
#[derive(Debug)]
#[allow(unused_variables)]
enum Alias{
    Mr,
    Mrs,
    Miss,
}

#[allow(dead_code)]
#[derive(Debug)]
#[allow(unused_variables)]
enum Departments{
    Physics,
    ComputersAndElectrical,
    Mechanical,
    Chemical,
    ComputerScience,

}
#[allow(dead_code)]
#[derive(Debug)]
#[allow(unused_variables)]
struct Users {
    alias:Alias,
    name: String,
    age: u8,
    position: Position,
    department:Departments,
}
fn main() {
    let person_1:Vec<Users>=vec![
        Users{
            alias:Alias::Mr,
            name:String::from("Benjamin"),
            age:32,
            position:Position::Student { grade:Some(45),student_position:StudentPosition::None} ,
            department:Departments::Physics,
        },

        Users{
            alias:Alias::Mr,
            name:String::from("Carson"),
            age:32,
            position:Position::Student { grade:Some(55),student_position:StudentPosition::Prefect} ,
            department:Departments::Mechanical,
        },
        Users{
            alias:Alias::Mrs,
            name:String::from("Victoria"),
            age:40,
            position:Position::Teacher{salary:4_000_000.0},
            department:Departments::ComputerScience,
        },
        Users{
            alias:Alias::Mr,
            name:String::from("Benjamin"),
            age:32,
            position:Position::Student { grade:Some(45),student_position:StudentPosition::ClassCaptain} ,
            department:Departments::ComputerScience
        },

        Users{
            alias:Alias::Mrs,
            name:String::from("Amuta"),
            age:32,
            position:Position::Teacher { salary: 300_000.0 },
            department:Departments::ComputersAndElectrical,
        }
    ];
}

impl Users {
    fn new(alias:Alias,name:String,age:u8,position:Position,department:Departments)->Self{
        Self { alias, name, age, position, department }
    }
    fn display_information(&self) {
        println!(
            "{:?},Name:{}\nage:{},position:{:?}",
            self.alias, self.name, self.age, self.position
        )
    }
    fn find_user(user_list:&Vec<Users>,given_name:&String)->Result<String,String>{
    // fn find_user(user_list:&Vec<Users>,given_name:&String)->Result<(),String>{
        for (_index,person) in user_list.iter().enumerate(){
            if (*given_name)==person.name{
                return Ok(person.name.clone());
            }


        }
        return Err(String::from("invalid entry"));
    }
}
