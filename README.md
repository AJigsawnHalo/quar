# Quar
A basic QR code reader and encoder for the terminal.
### Warning
Currently Quar only reads QR codes from an image. Camera support is planned but isn't a priority.
## Usage
To encode a message onto a QR code:
```
quar encode -i "input message" -o filename.png
```
To decode a QR code image:
```
quar decode -i /path/to/QR/image/
```
## License
Quar is licensed under the [MIT License](https://github.com/AJigsawnHalo/quar/blob/main/LICENSE)