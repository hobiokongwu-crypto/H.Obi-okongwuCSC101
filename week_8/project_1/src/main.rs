type LevelRule<'a> = (&'a str, u8, u8, Vec<&'a str>);


fn get_job_levels()-> Vec<LevelRule<'static>>{
    vec![
    ("APS 1-2",0,2, vec!["Intern","Paralegal Placement"]),
    ("APS 3-5",3,4, vec!["Administrator","Research Assistant","Junior Associate","Classroom Teacher"]),
    ("APS 5-8",5,8, vec!["Senior Administrator","PhD Candidate","Associate","Snr Teacher"]),
    ("EL1 8-10",9,10, vec!["Office Manager","Post-Doc Researcher","Senior Associate 1-2","Leading Teacher"]),
    ("EL2 10-13",11,13, vec!["Director","Senior Lecturer","Senior Associate 3-4","Deputy Principal"]),
    ("SES", 14,u8::MAX,vec!["CEO","Dean","Partner","Principal"]),
    ]
}

fn check_staff_level(job_role:&str,experience_years: u8) {
    let job_levels = get_job_levels();
    let mut matching_rule_found = false;

    println!("\n---Staff Level Verification---");
    println!("Staff Role: {}",job_role);
    println!("Experience: {} years",experience_years);

    for (levels_range, min_exp, max_exp, roles)in job_levels.iter() {
        if experience_years>= *min_exp && experience_years <= *max_exp {
            if roles.iter().any(|&role| role == job_role) {
                println!("👌 Validation Success!");
                println!("The Staff Member with role '{}' is correctly placed at: {}", job_role, levels_range);
                matching_rule_found = true;
                break;
            } else {
                println!("👎 Validation Failed: Role mismatch.");
                println!("The experience ({}) suggests level {}, but role '{}' is not valid for this level.", experience_years, levels_range, job_role );
                matching_rule_found = true;
                break;
            }
        }
    } if !matching_rule_found {
        println!("👎 Validation Failed: Experience outside of the defined ranges or role isn't found.");
        println!("The input role or experience did not match any defined APS rule.");
    }
}

fn main() {
    check_staff_level("Associate",6);
    check_staff_level("Leading Teacher",10);
    check_staff_level("Partner",15);
    check_staff_level("Partner",3);
    check_staff_level("Data Scientist",7);
}