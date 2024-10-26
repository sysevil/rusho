# Rust Shodan CLI - Rusho 🦀🏴‍☠️

A Rust-based CLI utility for querying subdomains and API credits on Shodan, with automatic key rotation to maximize usage.

## Features

- **Credit Check**: Before querying subdomains, the CLI checks if the API key has available credits to prevent request errors.
- **Subdomain Lookup**: Queries subdomains for a specific domain using the Shodan API.
- **Key Rotation**: Allows rotation between multiple Shodan keys specified in a `.txt` file, ensuring that only keys with credits are used.
- **Flexible Configuration**:
  - **Default Key Configuration**: If no key file is provided, the application will use the default API key configured in `$HOME/.config/rusho/config.yaml` or passed by the user via flag.
  - **Output File**: Specifies a custom output file or defaults to a file named `<domain>.rusho`.

## Installation

Clone the repository and compile the binary with Rust installed:

```bash
git clone https://github.com/your_username/rusho.git
cd rusho
cargo build --release
```

The binary will be available in target/release/rusho.

Usage

./rusho [FLAGS] -d <domain>

Flags and Arguments

Flag	Description	Example
-d, --domain	(Required) Defines the domain for subdomain lookup.	-d example.com
-c, --config	(Optional) Path to a YAML configuration file with the default Shodan key.	-c /path/to/config.yaml
-x, --key_file	(Optional) .txt file with multiple Shodan keys, one per line, for key rotation.	-x /path/to/keys.txt
-o, --output	(Optional) Sets the output file to save the found subdomains.	-o results.txt

Examples

1.	Using a Single Shodan Key (configured in the YAML file):

```bash
./rusho -d example.com
```

2.	Using a File with Multiple Keys for Rotation:

```bash
./rusho -d example.com -x keys.txt
```

3.	Specifying a Configuration File and an Output File:

```bash
./rusho -d example.com -c custom_config.yaml -o output.txt
```

YAML Configuration File Format

To define a default key, the YAML file (config.yaml) should have the following structure:

key: "your_shodan_key_here"

Key File Format (keys.txt)

The keys.txt file should contain one Shodan key per line:

shodan_key_1
shodan_key_2
shodan_key_3
