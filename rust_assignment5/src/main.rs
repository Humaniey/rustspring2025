// // Create a struct Student
// #[derive(Debug)]
// struct Student {
//     name: String,
//     major: String,
// }

// // Higher order functions: update_majors
// // fn update_majors(students: &mut [Student], operation: fn(&mut Student)) {
// //     for major in students.iter_mut() {
// //         operation(major);
// //     }
// // }

// fn update_majors(collection: Vec<Student>, behavior:fn(&mut Student,String)) {
//     for major in collection {
//         behavior(major);
//     }
// }
// // First Order functions, assign_major(student, major_declared)
// fn assign_major(student: &mut Student) {
//     student.major = "CS".to_string();
// }

// // Helper Methods
// fn print_students(student: &[Student]) {
//     for (i, student) in student.iter().enumerate() {
//         if i > 0 {
//             print!(", ");
//         }
//         println!();
//         print!("{:?}", student);
//     }
//     println!();
// }

// fn main() {
//     // Create a vector of students1,2,3 and update all students major
//     let mut students = vec![
//         Student {name: "Nathan".to_string(), major: "CE".to_string()},
//         Student {name: "Joel".to_string(), major: "EE".to_string()},
//         Student {name: "Eddy".to_string(), major: "SE".to_string()},
//         Student {name: "Raul".to_string(), major: "ME".to_string()},
//     ];

//     print_students(&students);
//     update_majors(&mut students, assign_major);
//     print_students(&students);
// }



//In class Assignment

// Create a struct Student (major)
#[derive(Debug)]
struct Student {
    major: String,
}

// First order functions, assig_major(student, major_declared)
fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

// Higher order functions update major
fn update_majors(collection: &mut Vec<Student>, behavior: fn(&mut Student, String)) {
    for mut student in collection.iter() {
        behavior(&student, "Computer Science".to_string());
    }
}
// Helper Method
fn print_students(student: &[Student]) {
    for (i, student) in student.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        println!();
        print!("{:?}", student);
    }
    println!();
}

fn main() {
    //Create a vector of students 1,2,3 and update all students major
    let mut students = vec![
        Student { major: String::new() },
        Student { major: String::new() },
        Student { major: String::new() },
    ];
    
    // Update majors using the higher-order function
    update_majors(&mut students, assign_major);
    print_students(&students);
    // // Print updated majors to confirm
    // for (i, student) in students.iter().enumerate() {
    //     println!("Student {}: Major = {}", i + 1, student.major);
    // }
}