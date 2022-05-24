__author__ = 'Benjamin M. Singleton'
__date__   = '24 May 2022'
__version  = '0.1.0'
from tqdm import tqdm

desired_ips_filename = 'desired_ips.csv'
total_ips_filename = 'total_ips.csv'


def get_lines(filename: str) -> list:
    with open(filename, 'r') as input_file:
        lines = input_file.read()
    lines = lines.trim().split('\n')
    return lines


def parse_ip_entries(ip_entries: list) -> list:
    parsed_ips = list()
    for each_entry in tqdm(ip_entries):
        if '-' in each_entry:
            # dashed range
            continue
        elif '/' in each_entry:
            # CIDR block
            continue
        else:
            # individual IP address
            continue
    return parsed_ips


def print_with_time(message: str) -> None:
    pass


def a_in_b(a: set, b: set) -> bool:
    return a.issubset(b)


def main():
    # import the entries
    desired_ips = get_lines(desired_ips_filename)
    total_ips = get_lines(desired_ips_filename)

    # expand both lists into individual IP addresses
    desired_ips = parse_ip_entries(desired_ips)
    total_ips = parse_ip_entries(total_ips)

    # convert each list to a set
    desired_ips = set(desired_ips)
    total_ips = set(total_ips)
    if a_in_b(desired_ips, total_ips):
        print("It's a match!")
    else:
        print("No luck.")



if __name__ == '__main__':
    main()
