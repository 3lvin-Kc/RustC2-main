⚠️ Disclaimer: This project is intended strictly for educational and research purposes only. Unauthorized use may violate local laws and regulations. Use responsibly.

🛠️ Rust-Based C2 (Command and Control)
This is a lightweight Command and Control (C2) tool built in Rust, designed with basic yet functional features for learning and experimentation.

📚 Available Commands (C2 Prompt)


<args> = required     [args] = optional

help                          Show this help menu
shell <cmd>                   Execute a local shell command
revshell <lang> [ip] [port]   Generate a reverse shell in a specific language
                              Examples: bash, nc, curl, php, powershell, python

connection <raw|client>       Switch between raw socket or client mode
                              (Toggles with no args; defaults to client)

-------------------------------------------------------------
Commands available when a client is connected:
-------------------------------------------------------------

list                          List all active connections
cmd <ID> <command>            Send a shell command to a connected host
psh <ID> <command>            Send a PowerShell command to a host
spawn <ID>                    Start an interactive shell session
import-psh <ID> <file>        Import a PowerShell script to a host
run-psh <ID> <Function>       Run a function from an imported PowerShell script
inject <ID> <Path> <args>     Execute an EXE or DLL in memory

upload <ID> <file> <dest>     Upload a file to the target machine
download <ID> <file> <dest>   Download a file from the target machine
portscan <ID> <IP> <N1> <N2>  Perform a port scan on a remote host
kill <ID>                     Terminate a beacon on the host
exit                          Close all sessions and exit (Ctrl+D)


''''''''''''''''''''''''''''''''''''

 Build Instructions
To compile the C2 tool:  

cargo build --release

The compiled binaries will be available in the target/release/ directory.

💬 Notes
The tool is designed for ease of use and learning purposes.

Contributions and PRs are very welcome!   

