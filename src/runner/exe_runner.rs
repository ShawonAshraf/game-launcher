use std::{path::Path, process::Command};


/// Represents an executor for running external executable files.
///
/// This struct encapsulates the functionality needed to execute external
/// programs, allowing users to specify which executable to run and manage its lifecycle.

pub struct ExeRunner {
    current_exe: String,
}

impl ExeRunner {
    /// Creates a new instance of `ExeRunner`.
    ///
    /// This function initializes a new `ExeRunner` with the specified executable path.
    /// It first checks if the provided path is valid. If the path is invalid,
    /// it returns an error indicating that the path is invalid.
    ///
    /// # Arguments
    ///
    /// * `exe_path` - A string slice that holds the path to the executable file.
    ///
    /// # Returns
    ///
    /// * `Result<ExeRunner, &'static str>` - A result containing either a new instance of `ExeRunner`
    ///   or an error message if the provided path is invalid.
    pub fn new(exe_path: String) -> Result<ExeRunner, &'static str> {
        if Self::check_for_valid_path(&exe_path) == false {
            return Err("Invalid Path");
        }

        Ok(ExeRunner { current_exe: exe_path })
    }

    /// Checks if the provided path is valid.
    ///
    /// This function takes a string slice representing a file or directory path
    /// and checks if it exists on the filesystem. It returns `true` if the path
    /// exists, otherwise `false`.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to be checked.
    ///
    /// # Returns
    ///
    /// * `bool` - `true` if the path exists, `false` otherwise.
    pub fn check_for_valid_path(path: &String) -> bool {
        Path::new(path).exists()
    }


    /// Runs the currently set executable.
    ///
    /// This function attempts to execute the executable stored in `current_exe`.
    /// It prints a message indicating which executable is being run and then
    /// uses the `Command` struct from the `std::process` module to spawn the
    /// executable. If the execution fails, it will panic with an appropriate message.
    ///
    /// # Returns
    ///
    /// * `u32` - The process ID of the spawned executable.
    pub fn run_exe(&self) -> u32 {
        println!("Running exe: {}", &self.current_exe);

        let status = Command::new(&self.current_exe)
            .spawn()
            .expect("The executable should run.");

        status.id()
    }
}