# Funcion approximator
This program creates a polynomial function that passes through points specified in a file.

For exmaple the file bellow gives the following output:
```
0, 0
1,1
2,4
3,9
```
```shell
$ cargo run input.txt
f(x) = x( 0.5(x-2)(x-3) -2(x-1)(x-3) + 1.5(x-1)(x-2))
```
