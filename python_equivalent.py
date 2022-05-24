__author__ = 'Benjamin M. Singleton'
__date__   = '24 May 2022'
__version  = '0.1.0'

from tqdm import tqdm
from datetime import datetime
import ipaddress

desired_ips_filename = 'desired_ips.csv'
total_ips_filename = 'total_ips.csv'


def get_lines(filename: str) -> list:
    with open(filename, 'r') as input_file:
        lines = input_file.read()
    lines = lines.strip().split('\n')
    return lines


def parse_ip_entries(ip_entries: list) -> list:
    parsed_ips = set()
    for each_entry in tqdm(ip_entries):
        if '-' in each_entry:
            # dashed range
            lower_bound, upper_bound = [int(ipaddress.IPv4Address(x)) for x in each_entry.split('-')]
            for x in range(lower_bound, upper_bound+1):
                parsed_ips.add(x)
        elif '/' in each_entry:
            # CIDR block
            network = ipaddress.IPv4Network(each_entry)
            lower_bound = int(ipaddress.IPv4Address(network.network_address))
            upper_bound = int(ipaddress.IPv4Address(network.broadcast_address))
            for x in range(lower_bound, upper_bound+1):
                parsed_ips.add(x)
        else:
            # individual IP address
            parsed_ips.add(int(ipaddress.IPv4Address(each_entry)))
    return parsed_ips


def print_with_time(message: str) -> None:
    print(f"{datetime.now().hour:02}:{datetime.now().minute:02}:{datetime.now().second:02}: " + message)


def a_in_b(a: set, b: set) -> bool:
    return a.issubset(b)


def main():
    # import the entries
    print_with_time(f"Loading IP range A from {desired_ips_filename}")
    desired_ips = get_lines(desired_ips_filename)
    print_with_time(f"Loading IP range B from {total_ips_filename}")
    total_ips = get_lines(total_ips_filename)

    # expand both lists into individual IP addresses
    print_with_time("Parsing the desired IPs.")
    desired_ips = parse_ip_entries(desired_ips)
    print_with_time("Parsing the total IPs.")
    total_ips = parse_ip_entries(total_ips)

    # convert each list to a set
    print_with_time("Converting list A to a set.")
    desired_ips = set(desired_ips)
    print_with_time("Converting list B to a set.")
    total_ips = set(total_ips)
    print_with_time("Checking if list A is in list B.")
    if a_in_b(desired_ips, total_ips):
        print_with_time("It's a match!")
    else:
        print_with_time("No luck.")


if __name__ == '__main__':
    main()
