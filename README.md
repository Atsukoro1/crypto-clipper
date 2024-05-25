# CryptoClipper

CryptoClipper is a super-efficient and probably best open-source stealthy cryptocurrency address manipulation tool designed to replace clipboard addresses with your own.

## ⚠️ Warning
This project is only meant to be used for educational purposes. I **do not take** any responsibility for action taken with this tool.

## ⭐ Features

- **Persistence** - Once it's on a system, CryptoClipper stays running even when the PC is restarted.
- **Mutex** - CryptoClipper uses mutex mechanisms to prevent multiple instances of itself from running on the same system.
- **0 CPU usage** - This tool is incredibly lightweight and won't bog down the system it's running on.
- **Small** - This program only takes like 2,3 mb of disk space, it only uses 2 crates
- **Force UAC** - It will force user to give this program administrator permissions
- **Stealthy** - It's not easy to find the virus on the victim's computer
- **6 Cryptocurrencies** - Our clipper supports BTC, XMR, DGE, LTC, ETH and Bitcoin cash
- **Windows defender bypass** - Clipper will add itself to windows defender exclusions once it executes
- **TODO: XOR** - Encrypt all strings at compile time and decompile at runtime
- **TODO: Virustotal bypass** - Ignore all Virustotal bots & Detect VM usage

## ⚙️ Installing

1. Open main.rs file and replace the addresses with your own
2. Compile the program using `cargo build`
3. Now you have your own build target/debug folder

## ⚙️ Configuration
All variables that are required for this project to work correctly are in the main.rs file.

### Variable explanation
1. FILE_NAME - This is the name of the executable file that will be persisted as a hidden file
2. MUTEX - Random string that is required for clipper to check that only one instance is running on system
3. FOLDER_NAME - You can configure under which folder will the clipper be hidden
4. ... Other crypto addresses - put your own crypto adresses here

## 🤝 Contributing to CryptoClipper

If you want to contribute to CryptoClipper, follow these steps:

1. Fork this repository.
2. Create a branch: `git checkout -b <branch_name>`.
3. Make your changes and commit them: `git commit -m '<commit_message>'`
4. Push to the original branch: `git push origin <project_name>/<location>`
5. Create the pull request.

Alternatively see the GitHub documentation on [creating a pull request](https://help.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request).