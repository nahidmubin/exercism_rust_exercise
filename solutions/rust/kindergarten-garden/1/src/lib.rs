use std::{collections::HashMap};

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    //Create a hashmap for plants
    let plants = HashMap::from([('G', "grass"), ('C', "clover"), ('R', "radishes"), ('V', "violets")]);
    //create the students array
    let students = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
    //split the diagram in two row
    let diagram: Vec<&str> = diagram.split('\n').collect();
    let row1: Vec<char> = diagram[0].chars().collect();
    let row2: Vec<char> = diagram[1].chars().collect();

    //get the given students index
    let student_index = match students.iter().position(|&x| x == student){
        Some(i) => i,
        None => panic!("The {} doesn't exists", student)
    };

    //Create the vector for the plants of the given student
    let mut student_plants: Vec<&'static str> = Vec::new();

    //Get the plants from each row
    student_plants.push(plants.get(&row1[student_index * 2]).unwrap());
    student_plants.push(plants.get(&row1[student_index * 2 + 1]).unwrap());
    student_plants.push(plants.get(&row2[student_index * 2]).unwrap());
    student_plants.push(plants.get(&row2[student_index * 2 + 1]).unwrap());

    //return the final result
    student_plants
}
