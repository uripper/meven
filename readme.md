# Meven - Video to Audio Conversion Tool

Meven is a simple command line tool designed in Rust to help with converting video files to audio. It focuses on extracting audio from videos.

## Current Capabilities

At present, Meven is capable of converting files from these video formats: .mp4, .mkv, .flv, .avi, and .webm. The output from Meven is an .mp3 format audio file.

## Getting Started

To get started with Meven:

1. Download the latest release of Meven from the releases page.
2. Add the Meven executable to your PATH environment variable.

## Using Meven

To use Meven, open a terminal and navigate to the directory containing your Meven executable. Then, type:

```shell
meven [your-directory-path]
```

Replace [your-directory-path] with the path to the directory that contains the videos you want to convert.

Example:

```shell
meven C:\\Users\\Jerma985\\Minecraft\\Unreleased_Videos
```
Once you've entered this command, Meven will convert all the video files in that directory to .mp3 files. If you need a reminder of how to use Meven, type:

```shell
meven -h
```

This will display help information about how to use Meven.


## Future Plans
1. Add support for more video formats.
2.  Add support for more audio format outputs.
3.  Parallel processing of video files.

## Feedback and Contributions
If you have any suggestions for features or improvements for Meven, feel free to raise an issue or submit a pull request. All contributions to the project are welcome.
