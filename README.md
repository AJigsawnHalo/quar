# Quar
A basic QR code reader and encoder for the terminal.
### NOTICE:
Currently Quar only reads QR codes from an image. Camera support is planned but isn't a priority.
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
## License
Quar is licensed under the [MIT License](https://github.com/AJigsawnHalo/quar/blob/main/LICENSE)