# Scramble

Scramble letters in each word of given sentence keeping first and last letter in place.

## Installation

### Unix users (Linux, BSDs and MacOSX)

Unix users may download and install latest *scramble* release with command:

```bash
sh -c "$(curl https://sweetohm.net/dist/scramble/install)"
```

If *curl* is not installed on you system, you might run:

```bash
sh -c "$(wget -O - https://sweetohm.net/dist/scramble/install)"
```

**Note:** Some directories are protected, even as *root*, on **MacOSX** (since *El Capitan* release), thus you can't install *scramble* in */usr/bin* for instance.

### Binary package

Otherwise, you can download latest binary archive at <https://github.com/c4s4/scramble/releases>. Unzip the archive, put the binary of your platform somewhere in your *PATH* and rename it *scramble*.

## Usage

To scramble a sentence, just run:

```bash
$ scramble The grass is always greener on the other side of the fence
The grsas is aywlas gerneer on the oehtr sdie of the fcene
```

*Enjoy!*
