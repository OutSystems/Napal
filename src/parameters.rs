use std::env;
use chrono::{Datelike, Timelike, Utc};
use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};

pub enum TimeFormat {
    Seconds,
    Minutes
}
pub struct Parameters {
    pub base_directory: PathBuf,
    pub skip_parse: bool,
    pub width_per_point: u32,
    pub target_directory: PathBuf,
    pub x_axis: TimeFormat,
    pub data_time_format: String,
    pub wanted_metrics_file: PathBuf,
    pub plotter_config_file: PathBuf,
    pub plotter_colors_file: PathBuf,
}

impl Parameters {

    fn new(base_directory: PathBuf, skip_parse: bool, width_per_point: u32, target_directory: PathBuf, wanted_metrics_file: PathBuf, 
        x_axis: TimeFormat, data_time_format: String, plotter_config_file: PathBuf, plotter_colors_file: PathBuf) -> Result<Parameters, Box<dyn std::error::Error>> {
        fs::create_dir_all(&target_directory)
            .with_context(|| format!("Could not create directory {:?}", &target_directory))?;
        
        Ok(Parameters {
            base_directory,
            skip_parse,
            width_per_point,
            target_directory,
            wanted_metrics_file,
            x_axis,
            data_time_format,
            plotter_config_file,
            plotter_colors_file,
        })
    }

    #[cfg(not(debug_assertions))]
    fn get_base_path() -> PathBuf {
        let exe_path = env::current_exe().unwrap();
        let parent_exe_path = exe_path.parent().unwrap();
        parent_exe_path.to_owned()
    }

    #[cfg(debug_assertions)]
    fn get_base_path() -> PathBuf {
        PathBuf::new()
    }

    pub fn obtain() -> (Parameters, Vec<PathBuf>) {
        let base_path = Parameters::get_base_path();

        let mut skip_parse = false;
        let mut width_per_point = 1;
        let now = Utc::now();
        // Default target directory is based on time
        let mut target_directory = base_path.join("results/").join(format!("{}-{}-{}_{}-{}-{}", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second()));
        let mut data_time_format = "%m/%d/%Y %H:%M:%S.%f".to_string();
        let mut file_list: Vec<PathBuf> = Vec::new();
        let mut x_axis = TimeFormat::Seconds;
        let mut wanted_metrics_file = base_path.join("config/DefaultMetrics.txt");
        let mut plotter_config_file = base_path.join("config/DefaultPlotSettings.txt");
        let mut plotter_colors_file = base_path.join("config/DefaultPlotLineColors.txt");
    
        let args: Vec<String> = env::args().skip(1).collect();

        if args.len() == 0 {
            Parameters::help();
            std::process::exit(0) 
        }
    
        let mut i = 0;
        while i < args.len() {
            let current_arg = &args[i];

            match current_arg.to_lowercase().as_str() {
                "-s" | "-skipparse" => skip_parse = true,
                "-w" | "-widthperpoint" => { 
                    width_per_point = args.get(i + 1).unwrap().parse::<u32>().unwrap();
                    i += 1
                }
                "-t" | "-targetdir" => {
                    target_directory = PathBuf::from(args.get(i + 1).unwrap().clone());
                    i += 1
                }
                "-tf" | "-timeformat" => {
                    data_time_format = args.get(i + 1).unwrap().clone();
                    i += 1
                }
                "-ps" | "-plotsettings" => {
                    plotter_config_file = PathBuf::from(args.get(i + 1).unwrap());
                    i += 1

                }
                "-c" | "-colorsfile" => {
                    plotter_colors_file = PathBuf::from(args.get(i + 1).unwrap());
                    i += 1
                }
                "-wm" | "-wantedmetrics" => {
                    wanted_metrics_file = PathBuf::from(args.get(i + 1).unwrap());
                    i += 1
                }
                "-h" | "-help" => {
                    Parameters::help();
                    std::process::exit(0) 
                }
                "-xaxis" => {
                    let time_format_arg = args.get(i + 1).unwrap().clone();
                    x_axis = match time_format_arg.to_lowercase().as_str() {
                        "seconds" => TimeFormat::Seconds,
                        "minutes" => TimeFormat::Minutes,
                        _ => panic!("Wrong time format. Options are <seconds> or <minutes>")
                    };
                    i += 1    
                }

                _ => file_list.push(PathBuf::from(current_arg))
            }
    
            i += 1;
        }
        let param = Parameters::new(
            base_path,
            skip_parse, 
            width_per_point, 
            target_directory, 
            wanted_metrics_file, 
            x_axis, 
            data_time_format,
            plotter_config_file,
            plotter_colors_file
        ).unwrap();
    
        (param, file_list)
    }

    pub fn help() {
        println!("{}", r"
        This is a currently barebones help section. Every parameter is optional.
        Parameters:

        -s or -skipParse:
            Whether the .csv files should be parsed or not. It is required for them to be parsed at least once, so that
            they generate the .altered.csv file
            Default is false

        -w or -widthPerPoint:
            The width of the generated image per given point in each graph (x axis).
            Default is 1

        -t or -targetDir:
            The directory where the results will be saved
            Default is a new directory whose name is the current time.

        -tf or -timeFormat:
            The format for the date column. How to write the format: https://docs.rs/chrono/0.4.7/chrono/format/strftime/index.html
            Default is <%m/%d/%Y %H:%M:%S.%f> (example date: 05/02/2020 15:30:10.012)

        -ps or -plotSettings:
            The path for the file that contains the settings to be used when plotting
            Default: config/DefaultPlotSettings.txt
        
        -c or -colorsFile:
            The path for the file that contains a list of the colors to be used in the graphs (by order)
            Default: config/DefaultPlotLineColors.txt

        -wm or -wantedMetrics:
            The path for the file that contains which metrics are desired to be analyzed.
            Default: config/DefaultMetrics.txt

        -h or -help:
            Displays this information.");
    }

    pub fn print(&self) {
        if self.skip_parse {
            println!("Skipping file parsing");
        } else {
            println!("Will parse the files")
        }
        println!("Width per point is {}", self.width_per_point);
        println!("Target directory is {:?}", self.target_directory);
        println!("The analysed metrics file is {:?}", self.wanted_metrics_file);
        println!("The data time format is {}", self.data_time_format);
        println!("The Plot config file is {:?}", self.plotter_config_file);
        println!("The colors file is {:?}", self.plotter_colors_file);
        println!("Time format is {}", self.data_time_format);
        match self.x_axis {
            TimeFormat::Seconds => println!("Plot X axis will be in seconds"),
            TimeFormat::Minutes => println!("Plot X axis will be in minutes")
        }
    }
}