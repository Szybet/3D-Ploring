use log::debug;
use pretty_env_logger::env_logger;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    env_logger::init();

    // Read the config. it neads to be in the same directory. and the binary needs to be named Ploter-postscript
    let mut config_path: String = String::new();
    match env::current_exe() {
        Ok(exe_path) => config_path = exe_path.display().to_string(),
        Err(e) => println!("failed to get current exe path: {}", e),
    }
    config_path = config_path.replace("Ploter-postscript", "ploter.conf");
    let config_file = File::open(config_path).unwrap();
    let config_file = BufReader::new(&config_file);
    let config_lines = config_file.lines();
    // Declare the variables. If no one is found, make it 0
    let mut x_home: String = String::from("0");
    let mut y_home: String = String::from("0");
    let mut z_home: String = String::from("0");

    let mut x_move: String = String::from("0");
    let mut y_move: String = String::from("0");
    let mut z_move: String = String::from("0");

    let mut regex: String = String::new();
    let mut start_gcode_move: String = String::new();
    let mut start_gcode_home: String = String::new();

    // Ho through every line of the file
    for line in config_lines {
        let mut line_string: String = String::new();
        match line {
            Err(e) => line_string = String::from("#"),
            Ok(r) => line_string = r,
        }
        if line_string == "" {
            line_string = String::from("#");
        }
        let if_comment = line_string.chars().nth(0).unwrap().to_string();
        // If the line is a comment ( or empty ), ignore it
        if if_comment != "#" {
            if line_string.contains("Plotter_X_Move:") {
                x_move = line_string.replace("Plotter_X_Move:", "");
            }
            if line_string.contains("Plotter_Y_Move:") {
                y_move = line_string.replace("Plotter_Y_Move:", "");
            }
            if line_string.contains("Plotter_Z_Move:") {
                z_move = line_string.replace("Plotter_Z_Move:", "");
            }
            if line_string.contains("Plotter_X_Home:") {
                x_home = line_string.replace("Plotter_X_Home:", "");
            }
            if line_string.contains("Plotter_Y_Home:") {
                y_home = line_string.replace("Plotter_Y_Home:", "");
            }
            if line_string.contains("Plotter_Z_Home:") {
                z_home = line_string.replace("Plotter_Z_Home:", "");
            }
            if line_string.contains("Extruder_Regex:") {
                regex = line_string.replace("Extruder_Regex:", "");
            }
            if line_string.contains("Starting_Gcode_Move:") {
                start_gcode_move = line_string.replace("Starting_Gcode_Move:", "");
            }
            if line_string.contains("Starting_Gcode_Home:") {
                start_gcode_home = line_string.replace("Starting_Gcode_Home:", "");
            }
        }
    }
    // Calculate the move_commands
    let move_command = format!("G1 X{} Y{} Z{}", x_move, y_move, z_move);
    let home_command = format!("G92 X{} Y{} Z{}", x_home, y_home, z_home);

    // Get the information from slicer
    let arguments: Vec<String> = env::args().collect();
    let path_gcode = arguments[1].clone();
    // Open the file
    let mut gcode_file = File::open(&path_gcode).unwrap();
    let mut gcode_data = String::new();
    gcode_file.read_to_string(&mut gcode_data);
    drop(gcode_file); // Close the file early
    let gcode_data = gcode_data.replacen(&start_gcode_move, &move_command, 1);
    let gcode_data = gcode_data.replacen(&start_gcode_home, &home_command, 1);

    let regex = Regex::new(&regex).unwrap();
    let gcode_data = regex.replace_all(&gcode_data, "");
    debug!("{}", gcode_data);

    let mut dst = File::create(&path_gcode).unwrap();
    dst.write(gcode_data.as_bytes()).unwrap();
}
