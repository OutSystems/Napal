# Napal
A faster Performance Analysis of Logs

This is currently a very barebones README.md, will add more information soon.


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
    Displays this information.