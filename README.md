## Audio bit converter

This is a simple CLI tool for converting WAV files to either 8,16 or 24bit. 


### Example

To convert a file simply use the command:

`audio_bit_converter --filename <filename>`

This will create a 16 bit version of the file in the same directory as the file.
There are optional arguments as --bits <bits> to specify the number of bits wanted for conversion, and
a verbose flag that can be either true or false, which provides some conversion info.

`audio_bit_converter --filename <filename> --bits 16 --verbose true`





