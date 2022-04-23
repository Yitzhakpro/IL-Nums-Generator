<div align="center">

# IL Phone Numbers Generator

</div>

## Usage

---

To view all options run:

```bash
$ ./il_numbers_generator --help
```

<br/>

Simple usage of creating all '050' combinations:

```bash
$ ./il_numbers_generator -p 050
```

this will result the file (il_nums.txt):

```
0500000001
0500000002
0500000003
0500000004
0500000005
0500000006
0500000007
...
```

**Note: you can enter multiple prefixes seperated by comma/space**

```bash
$ ./il_numbers_generator -p 050 054 052
```

```bash
$ ./il_numbers_generator -p 050, 054, 052
```

<br/>

Example of changing final output file name:

```bash
$ ./il_numbers_generator -p 050 -o somthing_else.txt
```

this will result the file (somthing_else.txt):

```
0500000001
0500000002
0500000003
0500000004
0500000005
0500000006
0500000007
...
```

<br/>

## Compile / Dev

---

Run the cargo:

```bash
$ cargo run -- --help
```

Compile:

```bash
$ cargo build --release
```

<br/>

## License

---

MIT © [Yitzhakpro](https://github.com/Yitzhakpro)
