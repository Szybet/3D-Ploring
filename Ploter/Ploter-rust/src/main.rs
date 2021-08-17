use log::debug;
use pretty_env_logger::env_logger;
use regex::Regex;
use serde_derive::Deserialize;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;

#[derive(Deserialize, Debug)]
struct Config {
    extruder_regex: String,
    replace_comment: String,
    custom_commands: String,
    t0_load: String,
    t0_unload: String,
    t1_load: String,
    t1_unload: String,
    t2_load: String,
    t2_unload: String,
    t3_load: String,
    t3_unload: String,
    t4_load: String,
    t4_unload: String,
    t5_load: String,
    t5_unload: String,
    t6_load: String,
    t6_unload: String,
    t7_load: String,
    t7_unload: String,
    t8_load: String,
    t8_unload: String,
}

fn main() {
    env_logger::init();

    // Get the config path. it neads to be in the same directory as the binary.
    let mut config_path: String = String::new();
    match env::current_exe() {
        Ok(exe_path) => config_path = exe_path.display().to_string(),
        Err(e) => println!("failed to get current exe path: {}", e),
    }
    let regex = Regex::new(&String::from(r"[A-Za-z0-9_?\-\.][A-Za-z0-9?.?-]+$")).unwrap();
    let config_path = regex.replace_all(&config_path, "ploter.conf").to_string();
    // Read the config, parse it to toml
    let mut config_file = File::open(config_path).unwrap();
    let mut config_file_string: String = String::new();
    config_file.read_to_string(&mut config_file_string).unwrap();
    let config_file_toml: Config = toml::from_str(&config_file_string).unwrap();

    // Get the information from slicer
    let arguments: Vec<String> = env::args().collect();
    let path_gcode = arguments[1].clone();

    // Open the gcode file
    let mut gcode_file = File::open(&path_gcode).unwrap();
    gcode_file.seek(SeekFrom::Start(0)).unwrap();
    let mut gcode_data = String::new();
    gcode_file.read_to_string(&mut gcode_data); // Makes that all ascii characters are seen, like \n
    drop(gcode_file); // Close the file early

    // Replace custom comment with custom commands
    let gcode_data = gcode_data.replacen(
        &config_file_toml.replace_comment,
        &config_file_toml.custom_commands,
        1,
    );

    // Delete all things that the regex points to
    let regex = Regex::new(&config_file_toml.extruder_regex).unwrap();
    let gcode_data = regex.replace_all(&gcode_data, "").to_string();
    let gcode_data_split = gcode_data.split("\n");
    
    // Manage multicolor tool changing
    let mut gcode_data_push: String = String::new();
    let mut first_change: bool = true;
    let mut previous_change: i32 = 0;
    for line_string in gcode_data_split {
        gcode_data_push.push_str(&manage_tool(
            line_string,
            &mut first_change,
            &mut previous_change,
            &config_file_toml,
        ));
    }

    // Save the file
    let mut file_write = File::create(&path_gcode).unwrap();
    file_write.write(gcode_data_push.as_bytes()).unwrap();
}

fn manage_tool(
    line_string: &str,
    first_change: &mut bool,
    previous_change: &mut i32,
    config_file_toml: &Config,
) -> String {
    let mut line_string_edited: String = String::new();
    if line_string.contains("T0 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t0_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t0_load.clone());
        }
        *previous_change = 0;
    } else if line_string.contains("T2 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t1_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t1_load.clone());
        }
        *previous_change = 1;
    } else if line_string.contains("T1 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t2_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t2_load.clone());
        }
        *previous_change = 2;
    } else if line_string.contains("T3 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t3_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t3_load.clone());
        }
        *previous_change = 3;
    } else if line_string.contains("T4 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t4_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t4_load.clone());
        }
        *previous_change = 4;
    } else if line_string.contains("T5 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t5_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t5_load.clone());
        }
        *previous_change = 5;
    } else if line_string.contains("T6 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t6_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t6_load.clone());
        }
        *previous_change = 6;
    } else if line_string.contains("T7 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t7_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t7_load.clone());
        }
        *previous_change = 7;
    } else if line_string.contains("T8 ; change extruder") {
        if first_change == &true {
            *first_change = false;
            line_string_edited = config_file_toml.t8_load.clone();
        } else {
            line_string_edited = previous_unload_return(previous_change, config_file_toml);
            line_string_edited.push_str(&config_file_toml.t8_load.clone());
        }
        *previous_change = 8;
    } else {
        line_string_edited.push_str(line_string);
        line_string_edited.push_str("\n");
    }
    line_string_edited
}

fn previous_unload_return(previous_change: &mut i32, config_file_toml: &Config) -> String {
    if previous_change == &0 {
        config_file_toml.t0_unload.clone()
    } else if previous_change == &1 {
        config_file_toml.t1_unload.clone()
    } else if previous_change == &2 {
        config_file_toml.t2_unload.clone()
    } else if previous_change == &3 {
        config_file_toml.t3_unload.clone()
    } else if previous_change == &4 {
        config_file_toml.t4_unload.clone()
    } else if previous_change == &5 {
        config_file_toml.t5_unload.clone()
    } else if previous_change == &6 {
        config_file_toml.t6_unload.clone()
    } else if previous_change == &7 {
        config_file_toml.t7_unload.clone()
    } else if previous_change == &8 {
        config_file_toml.t8_unload.clone()
    } else {
        String::from("PAUSE\n")
    }
}
