use std::fs;
use std::str::FromStr;
use indicatif::ProgressBar;
use std::collections::HashSet;
use chrono::Utc;
use clap::{App, Arg};

/// Represents a contiguous range of IPv4 addresses,
/// or a single address when start == end.
struct IPRange {
    start: u32,
    end: u32,
}

/// This utility takes two mandatory, positional arguments. These specify
/// the CSV files containing the subset and superset of IPs.
fn get_args() -> (String, String) {
    let matches = App::new("Compare IP addresses and ranges")
        .arg(Arg::with_name("a_file"))
        .arg(Arg::with_name("b_file"))
        .get_matches();
    let a_filename = matches.value_of_lossy("a_file").unwrap();
    let b_filename = matches.value_of_lossy("b_file").unwrap();
    (a_filename.to_string(), b_filename.to_string())
}

/// Given a filename, returns a Vec of Strings representing the
/// individual lines. There's no actual parsing or validation here.
fn read_ip_entries(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let ip_entries: Vec<String> = contents.trim().split("\n").map(|x| x.to_string()).collect();
    ip_entries
}

/// As the name implies, takes a String representing a dotted IPv4 address,
/// e.g., "192.168.1.1", and converts it to the 32-bit decimal
/// representation (3232235777.)
fn convert_ipv4_address(ip_address: &str) -> u32 {
    let mut encoded_address: u32 = 0;
    for (octet_num, each_octet) in ip_address.split(".").enumerate() {
        let each_octet = u32::from_str(each_octet).unwrap();
        encoded_address += u32::pow(256, (3 - octet_num).try_into().unwrap()) * each_octet;
    }
    encoded_address
}

/// Unused utility function, converts a 32-bit decimal reprsentation of an
/// IPv4 address to the standard dot-decimal String represenation humans
/// usually work with.
fn integer_to_ipv4_address(raw_address: u32) -> String {
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

/// Takes a Vec of Strings, which can represent individual IPv4
/// addresses, dashed ranges, or CIDR blocks.
/// Returns a HashSet of individual IP addresses, represented as 32-bit
/// integers.
fn _parse_ip_entries(ip_entries: Vec<String>) -> HashSet<u32> {
    let mut parsed: HashSet<u32> = HashSet::new();
    let bar = ProgressBar::new(ip_entries.len() as u64);
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
        // increment our progress bar with each IP address/range/CIDR we process.
        bar.inc(1);
    }
    bar.finish();
    parsed
}

/// Takes a Vec of Strings, which can represent individual IPv4
/// addresses, dashed ranges, or CIDR blocks.
/// Returns a Vector of IPRange structs.
fn parse_ip_entries_to_ranges(ip_entries: Vec<String>) -> Vec<IPRange> {
    let mut parsed: Vec<IPRange> = Vec::new();
    let bar = ProgressBar::new(ip_entries.len() as u64);
    for each_entry in ip_entries {
        if each_entry.contains("-") {
            // dashed IP range
            let lower_bound = each_entry.split("-").nth(0).unwrap().trim();
            let upper_bound = each_entry.split("-").nth(1).unwrap().trim();
            parsed.push(IPRange {
                start: convert_ipv4_address(lower_bound),
                end: convert_ipv4_address(upper_bound),
            });
        } else if each_entry.contains("/") {
            // CIDR range
            let mut lower_bound = convert_ipv4_address(&each_entry.split("/").nth(0).unwrap());
            let mask_bits = u32::from_str(each_entry.split("/").nth(1).unwrap()).unwrap();
            lower_bound = (lower_bound >> (32 - mask_bits)) << (32 - mask_bits);
            let upper_bound = lower_bound + (u32::pow(2, 32 - mask_bits) - 1);
            parsed.push(IPRange {
                start: lower_bound,
                end: upper_bound,
            });
        } else {
            // individual IP address
            parsed.push(IPRange {
                start: convert_ipv4_address(&each_entry),
                end: convert_ipv4_address(&each_entry),
            });
        }
        // increment our progress bar with each IP address/range/CIDR we process.
        bar.inc(1);
    }
    bar.finish();
    parsed
}

/// Checks if all of the IP addresses in a Vec of IPRanges are in another Vec
/// of IPRange.
fn a_in_b(a: Vec<IPRange>, b: Vec<IPRange>) -> bool {
    for each_a in a {
        let mut each_a_addresses: Vec<u32> = Vec::with_capacity((each_a.end - each_a.start + 1).try_into().unwrap());
        // We have to expand up the A ranges into individual IPs to make sure
        // address is somewhere in the B ranges/addresses.
        for x in each_a.start .. each_a.end + 1 {
            each_a_addresses.push(x);
        }
        for each_a_address in each_a_addresses {
            let mut found = false;
            for each_b in &b {
                // note that we never calculate the individual IP addresses in
                // each B range, we just need the start and end of each range
                // to see if a given A address is in it.
                if each_a_address >= each_b.start && each_a_address <= each_b.end {
                    found = true;
                    break;
                }
            }
            if !found {
                print_with_time(format!("Unmatched address {} from A entry {}-{}", integer_to_ipv4_address(each_a_address), integer_to_ipv4_address(each_a.start), integer_to_ipv4_address(each_a.end)).as_str());
                return false
            }
        }
    }
    true
}

/// Precede any console messages with a timestamp.
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
    let parsed_desired_ips = parse_ip_entries_to_ranges(raw_desired_ips);
    print_with_time("Parsing the total IPs.");
    let parsed_total_ips = parse_ip_entries_to_ranges(raw_total_ips);

    // check if all of the desired IPs are contained in the total IPs
    print_with_time("Checking if list A is in list B.");
    if a_in_b(parsed_desired_ips, parsed_total_ips) {
        print_with_time("It's a match!");
    } else {
        print_with_time("No luck.");
    }
}
