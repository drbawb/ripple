# ripple

a simple rpn calculator

---

## Building

`cargo build` once I unfsck the platform-specific dependencies

## Usage

* type a number (`([0-9] | '.')`) and press enter to push it onto the stack
* press `(+ | - | * | /)` to perform basic arithmetic on the two top-most elements
  * this pops two elements from the stack and pushes the result back onto the stack
* hit 'd' to discard the number on the top of the stack
* hit 'q' to exit the program

## TODO

* Fancy Mathematics
* more entry / display modes
  * e.g: fixed, scientific, dec/bin/hex/oct
* loads of stuff
