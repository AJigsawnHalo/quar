# Quar
A basic QR code reader and encoder for the terminal.

### NOTICE:
Currently Quar only reads QR codes from an image. Camera support is planned but isn't a priority.

## Installation
It is recommended to build and install Quar from source. There are binary files included in the [releases page](https://github.com/AJigsawnHalo/Quar/releases) but these are not tested to run on other systems.

### Build and Install
Installing Quar requires a Rust compiler and `cargo`. You can install them from [here](https://rustup.rs/).
1. Clone the repository using git or download a zip copy of the latest release from the [releases](https://github.com/AJigsawnHalo/Quar/releases)
 page.
```
git clone https://github.com/AJigsawnHalo/Quar.git
cd Quar
```
2. Install using:
 ```
 cargo install --path .
 ``` 

## Usage
To encode a message onto a QR code:
```
quar encode -i "input message" -o filename.png
```
To encode a file to a QR code:
```
quar encode -f /path/to/file.txt -o filename.png
```
To decode a QR code image:
```
quar decode -i /path/to/QR/image.png
```
To decode to a file:
```
quar decode -i /path/to/QR/image.png -o /path/to/file.txt
```

## License
Quar is licensed under the [MIT License](https://github.com/AJigsawnHalo/quar/blob/main/LICENSE).