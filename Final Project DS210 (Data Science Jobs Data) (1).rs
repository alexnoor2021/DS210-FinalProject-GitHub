use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let file = File::open("data_sci_jobs.csv").unwrap();
    let reader = BufReader::new(file);

    let mut salaries: HashMap<String, Vec<String>> = HashMap::new();
    let mut job_titles: HashMap<String, Vec<String>> = HashMap::new();
    let mut locations: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines().skip(1) {
        let record = line.unwrap();
        let fields: Vec<&str> = record.split(',').collect();

        let salary = fields[2].to_owned();
        let job_title = fields[0].to_owned();
        let location = fields[1].to_owned();

        salaries.entry(salary).or_default().push(record.to_owned());
        job_titles.entry(job_title).or_default().push(record.to_owned());
        locations.entry(location).or_default().push(record.to_owned());
    }

    println!("Salary:");
    for (key, value) in &salaries {
        println!("{}:", key);
        for record in value {
            println!("{}", record);
        }
    }

    println!("Job Title:");
    for (key, value) in &job_titles {
        println!("{}:", key);
        for record in value {
            println!("{}", record);
        }
    }

    println!("Location:");
    for (key, value) in &locations {
        println!("{}:", key);
        for record in value {
            println!("{}", record);
        }
    }
}


use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct JobListing {
    job_title: String,
    location: String,
    salary: u32,
}

impl JobListing {
    fn new(job_title: String, location: String, salary: u32) -> Self {
        Self {
            job_title,
            location,
            salary,
        }
    }
}

fn parse_salary(salary: &str) -> u32 {
    let salary = salary.replace("$", "").replace(",", "");
    salary.parse().unwrap()
}


let mut avg_salaries: HashMap<String, f32> = HashMap::new();

let job_titles: HashMap<String, Vec<String>> = HashMap::new();

for (job_title, listings) in &job_titles {
    let total_salary = listings
        .iter()
        .map(|listing| parse_salary(&listing.split(',').nth(2).unwrap()))
        .sum::<u32>() as f32;
    let count = listings.len() as f32;
    let avg_salary = total_salary / count;
    avg_salaries.insert(job_title.clone(), avg_salary);
}

println!("Average Starting Salaries:");
for (job_title, avg_salary) in &avg_salaries {
    println!("{}: ${:.2}", job_title, avg_salary);
}


use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct JobListing {
    job_title: String,
    location: String,
    salary: u32,
}

impl JobListing {
    fn new(job_title: String, location: String, salary: u32) -> Self {
        Self {
            job_title,
            location,
            salary,
        }
    }
}

fn main() {
    let file = File::open("data_sci_jobs.csv").unwrap();
    let reader = BufReader::new(file);

    let mut salaries: HashMap<String, Vec<String>> = HashMap::new();
    let mut job_titles: HashMap<String, Vec<String>> = HashMap::new();
    let mut locations: HashMap<String, Vec<String>> = HashMap::new();

    for line in reader.lines().skip(1) {
        let record = line.unwrap();
        let fields: Vec<&str> = record.split(',').collect();

        let salary = fields[2].parse::<u32>().unwrap();
        let job_title = fields[0].to_owned();
        let location = fields[1].to_owned();

        salaries.entry(salary.to_string()).or_default().push(record.to_owned());
        job_titles.entry(job_title.to_owned()).or_default().push(record.to_owned());
        locations.entry(location.to_owned()).or_default().push(record.to_owned());
    }

    // Calculate average salary for each location
    let mut avg_salaries: HashMap<String, f64> = HashMap::new();
    for (location, listings) in &locations {
        let mut total_salary: u32 = 0;
        let num_listings = listings.len();
        for listing in listings {
            let fields: Vec<&str> = listing.split(',').collect();
            total_salary += fields[2].parse::<u32>().unwrap();
        }
        let avg_salary = total_salary as f64 / num_listings as f64;
        avg_salaries.insert(location.to_owned(), avg_salary);
    }

    // Print results
    println!("Average Starting Salaries by Location:");
    for (location, avg_salary) in &avg_salaries {
        println!("{}: {:.2}", location, avg_salary);
    }
}













