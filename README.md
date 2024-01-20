# Function approximator
This program creates a polynomial function that passes through points specified in a file.

For example the file bellow gives the following output:
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

## How it works
Having a set of points $W(x_1) = y_1, W(x_2) = y_2$ and so on we can write a polynomial for each of them in the following way.
$$
W_1(x) = a_1(x-x_2)(x-x_3)... = y_1
W_2(x) = a_2(x-x_1)(x-x_3)... = y_2
...
$$
