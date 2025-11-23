
type DeveloperEntry<'a> = (&'a str, u32,&'a str);

fn get_developer_list() -> Vec<DeveloperEntry<'static>>{
    vec![
    ("Ngozi Ezeenwa",8, "Lagos, Nigeria"),
    ("Tunde Adekunle",12,"Abuja, Nigeria"),
    ("Chiamaka Okoro",5,"Port Harcourt, Nigeria"),
    ("Bayo Williams",15,"London,UK"),
    ("Amara Ibe",10,"Lagos, Nigeria"),
    ("Kunle Alabi",12,"Lagos, Nigeria"),
    ("Oluwa Toyin",3,"Abuja, Nigeria")]
}

fn find_max_experience<'a>(developers: &'a [DeveloperEntry<'a>])-> Option<DeveloperEntry<'a>>{
    developers.iter()
    .copied()
    .max_by_key(|&(_,years, _)|years)
}


fn main() {
    println!("---EY Developer Scouting Initiative (Nigeria)---");
    let developer_data = get_developer_list();
    println!("Total Candidates: {}", developer_data.len());
    println!("---Candidate List---");
    for (name,years,location) in &developer_data {
        println!("Name:{:<15}|Experience:{:>2} yrs| Location:{}",name,years,location);
    }
    println!("-----------------");
    let result = find_max_experience(&developer_data);

    match result{
        Some((name,years,location)) =>{
            println!("\n🎆 TOP CANDIDATE IDENTIFIED 🎆");
            println!("The candidates with the highest years of programming experience is:");
            println!("Name:{}",name);
            println!("Experience:{} years",years);
            println!("Location:{}",location);
            println!("\nConclusion: This person is the ideal candidate for the role.");
        }
        None =>{
            println!("\nNo developeer data found. The list is empty.");
        }
    }
}