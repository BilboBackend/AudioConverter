## Audio bit converter

This is a simple CLI tool for converting WAV files to either 8,16 or 24bit. 


### Example

To convert a file simply use the command:

`audio_bit_converter --filenames <filenames>`

This will create a 16 bit version of the file in the same directory as the file.

The --filenames flag takes multiple inputs either separated by spaces, but also support regex.

There are optional arguments as --bits <bits> to specify the number of bits, 8, 16 or 24, wanted for conversion, and
a verbose flag which provides some conversion info.

`audio_bit_converter --filename <filename> --bits 16 --verbose true`

If you want to specify a destination folder you can simply use the flag `--destination <path>`,  
which will be prepend the path to the filenames.





