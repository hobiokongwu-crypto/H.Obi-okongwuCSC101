use std::fs::File;
use std::io::Write;

fn main() {
    let filename = "commissioner_list.txt";

    let commissioners = ["Abiodun Alamutu Dauda", "Mustafa Azeem Bendu", "Okoroacha Titus Ogbona", "Adewale Jimoh Balogun", "Oluwole Faith Ezeh"];
    let zones = ["South West", "North East", "South South", "South West", "South East"];
    let ministries = ["Internal Affairs", "Justice", "Defense", "Power & Steel", "Petroleum"];
    let mut output_content = String::new();
    let header = format!("{:.<5}{:.<30}{:.<15}{:.<20}\n", "S/N", "COMMISSIONER", "ZONE", "MINISTRY");
    println!("\n--THE MINISTRIAL COMMISSION LIST--");
    println!("{}", header.trim_end());
    output_content.push_str(&header);
    output_content.push_str("--------------------------------------------------\n");


    for i in 0..commissioners.len() {
        let sn = i +1;
        let line = format!("{:.<5}{:.<30}{:.<15}{:.<20}\n", sn, commissioners[i], zones[i], ministries[i]);
        println!("{}", line.trim_end());
        output_content.push_str(&line);
    }
    let mut file = File::create(filename).expect("ERR:Failed to create the file!!");
    file.write_all(output_content.as_bytes()).expect("ERR:Failed to write the file!!");
    println!("\n🎉The Report Shall Be Saved As '{}'🎊", filename);
}
