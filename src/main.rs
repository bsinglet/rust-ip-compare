use std::fs;
use std::str::FromStr;
use std::collections::HashSet;
use chrono::Utc;
use clap::{App, Arg};

fn get_args() -> (String, String) {
    let matches = App::new("Compare IP addresses and ranges")
        .arg(Arg::with_name("a_file"))
        .arg(Arg::with_name("b_file"))
        .get_matches();
    let a_filename = matches.value_of_lossy("a_file").unwrap();
    let b_filename = matches.value_of_lossy("b_file").unwrap();
    (a_filename.to_string(), b_filename.to_string())
}

fn read_ip_entries(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let ip_entries: Vec<String> = contents.trim().split("\n").map(|x| x.to_string()).collect();
    ip_entries
}

fn convert_ipv4_address(ip_address: &str) -> u32 {
    let mut encoded_address: u32 = 0;
    for (octet_num, each_octet) in ip_address.split(".").enumerate() {
        let each_octet = u32::from_str(each_octet).unwrap();
        encoded_address += u32::pow(256, (3 - octet_num).try_into().unwrap()) * each_octet;
    }
    encoded_address
}

fn _integer_to_ipv4_address(raw_address: u32) -> String {
    let mut working_address = raw_address.clone();
    let mut encoded_address = String::new();
    for octet_num in 0..4 {
        let dividend: u32 = working_address / u32::pow(256, 3 - octet_num);
        encoded_address.push_str(&dividend.to_string());
        if octet_num < 3 {
            encoded_address.push_str(".")
        }
        working_address -= dividend * u32::pow(256, 3 - octet_num);
    }
    encoded_address
}

fn parse_ip_entries(ip_entries: Vec<String>) -> HashSet<u32> {
    let mut parsed: HashSet<u32> = HashSet::new();
    for each_entry in ip_entries {
        if each_entry.contains("-") {
            // dashed IP range
            let lower_bound = each_entry.split("-").nth(0).unwrap().trim();
            let upper_bound = each_entry.split("-").nth(1).unwrap().trim();
            let lower_bound = convert_ipv4_address(lower_bound);
            let upper_bound = convert_ipv4_address(upper_bound);
            for each_ip in lower_bound..upper_bound + 1 {
                parsed.insert(each_ip);
            }
        } else if each_entry.contains("/") {
            // CIDR range
            let mut lower_bound = convert_ipv4_address(&each_entry.split("/").nth(0).unwrap());
            let mask_bits = u32::from_str(each_entry.split("/").nth(1).unwrap()).unwrap();
            lower_bound = (lower_bound >> (32 - mask_bits)) << (32 - mask_bits);
            let upper_bound = lower_bound + (u32::pow(2, 32 - mask_bits) - 1);
            for each_ip in lower_bound..upper_bound + 1 {
                parsed.insert(each_ip);
            }
        } else {
            // individual IP address
            parsed.insert(convert_ipv4_address(&each_entry));
        }
    }
    parsed
}

fn print_with_time(message: &str) {
    println!("{}: {}", Utc::now().format("%T"), message);
}

fn main() {
    // load the data
    let (filename1, filename2) = get_args();
    print_with_time(format!("Loading IP range A from {}", filename1).as_str());
    let raw_desired_ips: Vec<String> = read_ip_entries(filename1.as_str());
    print_with_time(format!("Loading IP range B from {}", filename2).as_str());
    let raw_total_ips: Vec<String> = read_ip_entries(filename2.as_str());

    // convert the raw strings to individual IP addresses (expanding any ranges)
    print_with_time("Parsing the desired IPs.");
    let parsed_desired_ips = parse_ip_entries(raw_desired_ips);
    print_with_time("Parsing the total IPs.");
    let parsed_total_ips = parse_ip_entries(raw_total_ips);

    // check if all of the desired IPs are contained in the total IPs
    print_with_time("Checking if list A is in list B.");
    if parsed_desired_ips.is_subset(&parsed_total_ips) {
        print_with_time("It's a match!");
    } else {
        print_with_time("No luck.");
    }
}
