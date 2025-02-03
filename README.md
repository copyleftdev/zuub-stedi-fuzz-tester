# Zuub Stedi Fuzz Tester

_A high-performance Rust tool for generating realistic Stedi API responses and fuzz testing integrations_

---

## Overview

The **Zuub Stedi Fuzz Tester** is built to simulate real-world API behaviors with precision. By generating diverse and realistic responses for the Stedi API, this tool helps you uncover edge cases and vulnerabilities in your integrations before they hit production.

---

## Why Build This Tool?

- **Realistic API Simulation:**  
  Create lifelike responses that mimic the nuances of real API behavior, enabling you to test your systems under authentic conditions.

- **Robust Fuzz Testing:**  
  Automatically inject randomized and edge-case inputs to identify and fix potential issues, boosting your integration’s resilience.

- **High Performance & Safety:**  
  Written in Rust, the tool harnesses Rust’s speed and safety features, ensuring reliable and efficient testing.

- **Easy Configuration:**  
  Tweak server settings, logging, error rates, and more through simple TOML configuration files.

---

## Benefits

- **Enhanced Reliability:**  
  Catch integration bugs and performance issues early, ensuring smooth production deployments.

- **Improved Security:**  
  Expose vulnerabilities by testing with a wide range of input scenarios, reinforcing your system’s security posture.

- **Time Efficiency:**  
  Automate repetitive testing tasks, reducing manual effort and accelerating your development cycle.

- **Modular & Extensible:**  
  A clean, modular codebase allows for easy customization and scaling to fit your testing needs.

---

## Features

- **Realistic Stedi API Responses:**  
  Generate accurate responses with dynamic parameters to mimic live environments.

- **Comprehensive Integration Tests:**  
  A dedicated suite of tests (found in `tests/integration_tests.rs`) ensures your API interactions are robust.

- **Detailed Logging:**  
  Configurable logging levels help you trace and debug issues with precision.

- **Flexible Configuration:**  
  Use the provided `config/default.toml` and `config/test.toml` files to adjust server address, port, log levels, and more.

- **Modular Architecture:**  
  Separate modules handle core logic, configuration, API handling, and utility functions for a well-organized codebase.

---

## Installation

Ensure you have [Rust](https://www.rust-lang.org/) installed, then clone the repository and build the project:

```bash
# Clone the repository
git clone https://github.com/copyleftdev/zuub-stedi-fuzz-tester.git
cd zuub-stedi-fuzz-tester

# Build the project in release mode
cargo build --release
