# Resampler
Resampler is a resampling program that resamples any supported file at the desired ratio using the desired interpolation algorithm.

## Table of Contents
- [File Types](#file_types)
- [Interpolation Algorithms](#interpolation_algorithms)
- [Potential Features](#potential_features)
- [Building](#building)
- [Usage](#usage)
- [License](#license)

## File Types
Currently supported files types are:
- CSV: reads the first column, ignoring the first row (headers are often in the first row)

File types planned for inclusion in the near future include:
- WAV
- MP3
- PNG
- JPEG

## Interpolation Algorithms
Currently supported interpolation algorithms include:
- Piecewise constant
- Lerp
- Hermite (with tangent components)
- Catmull-Rom hermite

Interpolation algorithms planned for inclusion in the near future include:
- Cardinal hermite
- Interpolation filter

## Potential Features
Some potential features for the distant future include:
- GPU interpolation option

## Building
Build with cargo: ```cargo build```

## Usage
Run ```resampler [OPTIONS] <SOURCE> <INPUT_RATIO> <OUTPUT_RATIO>```
- ```<SOURCE>``` is the source file to retrieve data from
- ```<INPUT_RATIO>``` and ```<OUTPUT_RATIO>``` define the number of input samples to output samples

Run ```resampler --help``` for more options

## License
This project is under the [MIT License](LICENSE).
