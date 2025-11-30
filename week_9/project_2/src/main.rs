use std::fs::File;
use std::io::Write;

fn main() {
    let filename = "Student_details.txt";

    let students = vec![
    (1, "Adekunle,J.", "PAU/CSC/19/001","CSC", 400),
    (2, "Owen,C.", "PAU/INF/20/123","INF", 300),
    (3, "Muhammad,A.", "PAU/ACC/21/456","ACC", 200),
    (4, "Chijoke,A.", "PAU/FIN/22/789","FIN", 100),];

    let mut output_content = String::new();
    
    let header = format!("{:.<5}{:<20}{:.<15}{:.<10}{:.<5}\n", "S/N", "Name", "Matric No.", "Dept", "Level");
    println!("{}", header.trim_end());
    output_content.push_str(&header);
    output_content.push_str("----------------------------------------\n");

    for student in students.iter(){
        let line = format!("{:.<5}{:<20}{:.<15}{:.<10}{:.<5}\n", student.0, student.1, student.2, student.3, student.4);
        println!("{}",line.trim_end());
        output_content.push_str(&line);
    }
    let mut file = File::create(filename).expect("ERR:Failed to create the file!!!");
    file.write_all(output_content.as_bytes()).expect("ERR:Failed to write the file!!!");
    println!("\n👌Report saved to '{}'🎆", filename);
}