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
    Student {
        grade: Option<u8>,
        student_position: StudentPosition,
    },
    Teacher {
        salary: f64,
    },
    Principal,
    VicePrinciple,
}

#[allow(dead_code)]
#[derive(Debug)]
#[allow(unused_variables)]
enum Alias {
    Mr,
    Mrs,
    Miss,
}

#[allow(dead_code)]
#[derive(Debug)]
#[allow(unused_variables)]
enum Departments {
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
    alias: Alias,
    name: String,
    age: u8,
    position: Position,
    department: Departments,
}
fn main() {
    let mut person_1: Vec<Users> = vec![
        Users {
            alias: Alias::Mr,
            name: String::from("Benjamin"),
            age: 32,
            position: Position::Student {
                grade: Some(45),
                student_position: StudentPosition::None,
            },
            department: Departments::Physics,
        },
        Users {
            alias: Alias::Mr,
            name: String::from("Carson"),
            age: 32,
            position: Position::Student {
                grade: Some(55),
                student_position: StudentPosition::Prefect,
            },
            department: Departments::Mechanical,
        },
        Users {
            alias: Alias::Mrs,
            name: String::from("Victoria"),
            age: 40,
            position: Position::Teacher {
                salary: 4_000_000.0,
            },
            department: Departments::ComputerScience,
        },
        Users {
            alias: Alias::Mr,
            name: String::from("Benjamin"),
            age: 32,
            position: Position::Student {
                grade: Some(45),
                student_position: StudentPosition::ClassCaptain,
            },
            department: Departments::ComputerScience,
        },
        Users {
            alias: Alias::Mrs,
            name: String::from("Amuta"),
            age: 32,
            position: Position::Teacher { salary: 300_000.0 },
            department: Departments::ComputersAndElectrical,
        },
    ];
    // fn new(alias: Alias, name: String, age: u8, position: Position, department: Departments) -> Self
    let user_1 = Users::new(
        Alias::Mr,
        "Stephen".to_string(),
        84,
        Position::Principal,
        Departments::Chemical,
    );
    // let user_1=Users::builder()
    //     .alias(Alias::Mr)
    //     .name("Stephen".to_string())
    //     .age(84)
    //     .position(Position::Principal)
    //     .department(Departments::Chemical)
    person_1.push(user_1);
    // user_1.display_information();
    let given_name = "benjamin".to_string();
    // let result = Users::find_user(&person_1, &given_name).expect("invalid conversion");
    let user_2 = Users::new(
        Alias::Mr,
        "Stephen".to_string(),
        84,
        Position::Principal,
        Departments::Chemical,
    );
    let result = user_2.find_user(&person_1, &given_name);
    if let Some(res)=  result{
        println!("{:?}",res)
        // Some(result)=>println("found:{:?}",result),
        // None=>{}


    }
}

impl Users {
    fn new(
        alias: Alias,
        name: String,
        age: u8,
        position: Position,
        department: Departments,
    ) -> Self {
        Self {
            alias,
            name,
            age,
            position,
            department,
        }
    }
    #[allow(dead_code)]
    fn display_information(&self) {
        println!(
            "{:?},Name:{}\nage:{},position:{:?}",
            self.alias, self.name, self.age, self.position
        )
    }
    // fn find_user(user_list: &Vec<Users>, given_name: &String) -> Result<String, String> {
    fn find_user(&self, user_list: &Vec<Users>, given_name: &String) -> Option<&Users> {
        // fn find_user(user_list: &Vec<Users>, given_name: &String) -> Option<u8> {
        // let mut state = false;
        // fn find_user(user_list:&Vec<Users>,given_name:&String)->Result<(),String>{
        for person in user_list.iter() {
            if (*given_name) == person.name {
                return Some(self);
                // state =true;
                // if let Some(found)=match state{
                //     true=>println!("found name:");
            }
            // return Some(person.position.)
            // return Some(person.position::Student{grade})
            // return Ok(person.name.clone());
        }
        return None;
        // for (_index, person) in user_list.iter().enumerate() {
        //     if (*given_name) == person.name {
        //         return Some(*person.position)
        //         // return Ok(person.name.clone());
        //     }
    }
    // None
    // return Err(String::from("invalid entry"));
}
// }
