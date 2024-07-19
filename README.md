# Subnet Calculator

A simple command-line subnet calculator written in Rust.

## What it does

- Calculates subnet info based on a network address, CIDR, and number of hosts
- Handles multiple subnets
- Lets you input data manually or import from a CSV file
- Can save results as CSV or Markdown

## Getting Started

1. Make sure you have Rust installed. If not, grab it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

2. Clone this repo:
git clone https://github.com/yourusername/subnet-calculator.git
cd subnet-calculator

4. Build it:
cargo build --release

4. Run it:
./target/release/subnet-calculator

Alternatively, [precompiled binaries for Windows](https://github.com/LuMarans30/Subnetting-rust/releases/latest) are available.

## Usage

### Manual Input

1. Choose option 1
2. Type in your network address with CIDR (like 192.168.1.0/24)
3. Enter how many subnets you want
4. For each subnet, enter the number of hosts you need

### CSV Import

1. Choose option 2
2. Enter the path to your CSV file

Your CSV should look like this:<br />
(The first line is the initial network address with CIDR, then one line per subnet with the number of hosts)
```csv
<ip address>,<cidr>
<num_host_1>
<num_host_2>
...
<num_host_3>
```
Alternatively, you can use a slash ('/') as a delimiter:
```csv
<ip address>/<cidr>
<num_host_1>
<num_host_2>
...
<num_host_3>
```

## What You'll Get

For each subnet, you'll see:
- Network address
- Subnet mask
- CIDR
- Subnet class
- Broadcast address
- Gateway
- First and last usable host addresses
- Request number of hosts
- Actual number of hosts
- How many IP addresses are wasted

You can save all this info to a CSV or Markdown file if you want.