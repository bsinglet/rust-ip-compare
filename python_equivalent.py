__author__ = 'Benjamin M. Singleton'
__date__   = '24 May 2022'
__version  = '0.2.0'

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


def _parse_ip_entries(ip_entries: list) -> list:
    parsed_ips = list()
    for each_entry in tqdm(ip_entries):
        if '-' in each_entry:
            # dashed range
            lower_bound, upper_bound = [int(ipaddress.IPv4Address(x)) for x in each_entry.split('-')]
            parsed_ips.extend(range(lower_bound, upper_bound+1))
        elif '/' in each_entry:
            # CIDR block
            network = ipaddress.IPv4Network(each_entry)
            lower_bound = int(ipaddress.IPv4Address(network.network_address))
            upper_bound = int(ipaddress.IPv4Address(network.broadcast_address))
            parsed_ips.extend(range(lower_bound, upper_bound+1))
        else:
            # individual IP address
            parsed_ips.append(int(ipaddress.IPv4Address(each_entry)))
    return parsed_ips

def parse_ip_entries_to_ranges(ip_entries: list) -> list:
    parsed_ips = list()
    for each_entry in tqdm(ip_entries):
        if '-' in each_entry:
            # dashed range
            lower_bound, upper_bound = [int(ipaddress.IPv4Address(x)) for x in each_entry.split('-')]
            parsed_ips.append((lower_bound, upper_bound))
        elif '/' in each_entry:
            # CIDR block
            network = ipaddress.IPv4Network(each_entry)
            lower_bound = int(ipaddress.IPv4Address(network.network_address))
            upper_bound = int(ipaddress.IPv4Address(network.broadcast_address))
            parsed_ips.append((lower_bound, upper_bound))
        else:
            # individual IP address
            parsed_ips.append((int(ipaddress.IPv4Address(each_entry)), int(ipaddress.IPv4Address(each_entry))))
    return parsed_ips


def print_with_time(message: str) -> None:
    print(f"{datetime.now().hour:02}:{datetime.now().minute:02}:{datetime.now().second:02}: " + message)


def _a_in_b(a: set, b: set) -> bool:
    return a.issubset(b)

def a_in_b_with_ranges(a: list, b: list) -> bool:
    for each_a_range in tqdm(a):
        each_a_addresses = list(range(each_a_range[0], each_a_range[1]+1))
        for each_a_address in each_a_addresses:
            found = False
            for each_b_range in b:
                if each_a_address >= each_b_range[0] and each_a_address <= each_b_range[1]:
                    found = True
                    break
            if not found:
                print(f"Couldn't find address {each_a_address} from list A entry {each_a_range}.")
                return False
    return True


def main():
    # import the entries
    print_with_time(f"Loading IP range A from {desired_ips_filename}")
    desired_ips = get_lines(desired_ips_filename)
    print_with_time(f"Loading IP range B from {total_ips_filename}")
    total_ips = get_lines(total_ips_filename)

    # expand both lists into individual IP addresses
    print_with_time("Parsing the desired IPs.")
    desired_ips = parse_ip_entries_to_ranges(desired_ips)
    print_with_time("Parsing the total IPs.")
    total_ips = parse_ip_entries_to_ranges(total_ips)

    # convert each list to a set
    print_with_time("Checking if list A is in list B.")
    if a_in_b_with_ranges(desired_ips, total_ips):
        print_with_time("It's a match!")
    else:
        print_with_time("No luck.")


if __name__ == '__main__':
    main()
