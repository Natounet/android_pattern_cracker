# Android Pattern Cracker

![image](https://github.com/Natounet/Tools/assets/70477133/e20f5444-a33f-4058-a16f-09cc41f65b76)

**Author:** Nathan CORNELOUP  
**Version:** 0.1.0

## Overview

Android Pattern Crack is a Rust-powered app designed to crack Android Pattern `gesture.key` files, unlocking devices with pattern locks up to Android 5.1.
The gesture.key file contain the SHA1 hash of the pattern in an hexadecimal form.

## Installation

Once you have cloned this repository, go inside the folder
```
cd android_pattern_cracker
```

Then compile the binary using cargo :
```
cargo build --release
```

You will find your android_pattern_cracker binary inside :
```
target/build
```

You can then use the binary :
```
./android_pattern_cracker -f gesture.key
```

## Usage

To use Android Pattern Cracker, follow these simple steps:
Navigate to the directory where the `android_pattern_cracker` executable is located.

```
./android_pattern_cracker -f gesture.key
```

## Explanation

A little explanation on how Android store the pattern.

The pattern numbers on the pattern lock screen are the following : 
```
[ 1 ] [ 2 ] [ 3 ]
[ 4 ] [ 5 ] [ 6 ]
[ 7 ] [ 8 ] [ 9 ]
```

If your pattern is a "U", like this one ( you start top left and finish top right ):
```
[ X ] [   ] [ X ]
[ X ] [   ] [ X ]
[ X ] [ X ] [ X ]
```
Your pattern number would be 1478963.

But, Android encode this pattern as the following :
```
[ 0 ] [ 1 ] [ 2 ]
[ 3 ] [ 4 ] [ 5 ]
[ 6 ] [ 7 ] [ 8 ]
```

So, your pattern would be hashed as : 0367852

Since each digit is encoded in one byte : 00 03 06 07 08 05 02 

Then, Android generate the SHA-1 hash of this hexadecimal number : SHA1("\x00\x03\x06\x07\x08\x05\x02") =>  61 8B 58 9A A9 8D FE E7 43 E7 12 09 13 A0 66 5A 4A 5E 83 17
and store it inside the gesture.key file.

## Exemple

![Animation](https://github.com/Natounet/Tools/assets/70477133/d4d77b73-0083-48b2-870f-bb45edb35fa7)


## External ressources
[ForensicFocus blog](https://www.forensicfocus.com/articles/android-forensics-study-of-password-and-pattern-lock-protection/)

