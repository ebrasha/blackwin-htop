# BlackWin htop - User Guide

<div align="center">
  <img src="scr.jpg" alt="BlackWin htop">
</div>


## 🎤 README Translation
- [English](README.md)
- [فارسی](README.fa.md)


## Overview
Bring htop to Windows – a Rust-based system monitor for developers and power users on Windows and Linux. BlackWin htop is a cyberpunk-themed system monitor that provides real-time information about your system's performance. With its beautiful terminal user interface, you can monitor CPU usage, memory utilization, and manage processes efficiently.

## Features

### System Monitoring
- Real-time CPU usage monitoring for each core
- Memory usage statistics
- Process management with sorting and filtering
- Beautiful cyberpunk-themed interface

### Key Features
- Individual CPU core monitoring
- Process list with detailed information
- Process search functionality
- Process termination capability
- Memory usage tracking
- Cyberpunk color theme

## Installation

### Windows
1. Download the latest release from GitHub
2. Extract the zip file
3. Run `htop.exe`

### Building from Source
1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Clone and build:
   ```bash
   git clone https://github.com/ebrasha/blackwin-htop.git
   cd blackwin-htop
   cargo build --release
   ```
3. Run:
   ```bash
   cargo run --release
   ```

## Usage

### Basic Navigation
- Use `↑/↓` or `j/k` to navigate through the process list
- Press `q` to quit the application
- Press `F3` to search for processes
- Press `F9` to terminate the selected process

### Process Management
- Sort processes by:
  - `p`: Process ID
  - `n`: Process name
  - `c`: CPU usage
  - `m`: Memory usage
- Search processes:
  1. Press `F3` to enter search mode
  2. Type the process name
  3. Press `Enter` to confirm or `Esc` to cancel

### Display Information
- CPU usage is shown with color-coded bars:
  - Green: Low usage (0-50%)
  - Orange: Medium usage (50-80%)
  - Red: High usage (80-100%)
- Memory information shows:
  - Total memory
  - Used memory
  - Usage percentage

## Troubleshooting

### Common Issues

1. **Program doesn't start**
   - Ensure you have administrator privileges
   - Check if any antivirus is blocking the application
   - Verify Windows Defender settings

2. **High CPU Usage**
   - Increase the update interval
   - Close other resource-intensive applications
   - Check system load

3. **Display Issues**
   - Ensure terminal supports Unicode
   - Check terminal color support
   - Verify terminal size is sufficient

### Error Messages

- "Failed to get system information":
  - Run as administrator
  - Check system permissions

- "Unable to terminate process":
  - Verify you have sufficient privileges
  - Process might be protected by system

## Support

### Getting Help
- GitHub Issues: [BlackWin Issues](https://github.com/ebrasha/blackwin-htop/issues)
- Email: Prof.Shafiei@Gmail.com

### Reporting Bugs
When reporting bugs, please include:
1. Operating system version
2. Steps to reproduce
3. Expected vs actual behavior
4. Error messages if any
5. Screenshots if applicable

## FAQ

**Q: Why does BlackWin htop show different CPU usage than Task Manager?**
A: Different tools may use different methods to calculate CPU usage. BlackWin htop focuses on physical cores for more accurate representation.

**Q: Can I customize the colors?**
A: Currently, the cyberpunk theme is fixed, but theme customization is planned for future releases.

**Q: Does it affect system performance?**
A: BlackWin htop is designed to be lightweight and efficient, with minimal impact on system resources.

**Q: Why can't I see all processes?**
A: Some processes might require elevated privileges. Try running the application as administrator.

## Tips and Tricks

1. **Efficient Navigation**
   - Use `Home/End` to jump to list start/end
   - Use `PageUp/PageDown` for faster scrolling

2. **Process Management**
   - Sort by CPU usage to identify resource-heavy processes
   - Use search to quickly find specific processes

3. **Performance Monitoring**
   - Monitor CPU cores individually for better insight
   - Check memory usage trends over time

## Updates and Upgrades

### Checking for Updates
1. Visit the GitHub repository
2. Check the releases page
3. Download and install the latest version

### Update Process
1. Close the current instance
2. Download the new version
3. Replace the old executable
4. Run the new version

## Security Notes

- Always verify downloads from official sources
- Be cautious when terminating unknown processes
- Run with appropriate privileges
- Keep the application updated 

## ❤️ Donation
If you find this project helpful and would like to support further development, please consider making a donation:
- [Donate Here](https://ebrasha.com/abdal-donation)

## 🤵 Programmer
Handcrafted with Passion by **Ebrahim Shafiei (EbraSha)**
- **E-Mail**: Prof.Shafiei@Gmail.com
- **Telegram**: [@ProfShafiei](https://t.me/ProfShafiei)

## 📜 License
This project is licensed under the GPLv2 or later License. 