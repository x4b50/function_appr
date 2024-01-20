# Function approximator
This program creates a polynomial function that passes through all the points specified in a file. It is also recommended to use graphical calculators, like desmos, to check the accuracy, especially with low data density.

For example the file bellow gives the following output:
```
0, 0
1,1
2,4
3,9
```
```shell
$ cargo run input.txt
f(x) =
x( 0.5(x-2)(x-3) -2(x-1)(x-3) + 1.5(x-1)(x-2))
```

## How it works
Having a set of points $W(x_1) = y_1, W(x_2) = y_2$ and so on we can write a polynomial for each of them in the following way.
```math
W_1(x) = a_1(x-x_2)(x-x_3)...(x-x_n) = y_1
```
```math
W_2(x) = a_2(x-x_1)(x-x_3)...(x-x_n) = y_2
```
```math
...
```
```math
W_n(x) = a_n(x-x_1)(x-x_2)...(x-x_{n-1}) = y_n
```

After that we need to calculate the $a$ coefficients by supplying $x$ values to corresponding polynomials and dividing the expected $y$ outputs by them.
```math
a_1 = \frac{y_1}{(x_1-x_2)(x_1-x_3)...(x_1-x_n)}
```
```math
a_2 = \frac{y_2}{(x_2-x_1)(x_2-x_3)...(x_2-x_n)}
```
```math
...
```
```math
a_n = \frac{y_n}{(x_n-x_1)(x_n-x_2)...(x_n-x_{n-1})}
```

Having calculated the coefficients we can sum up all of the polynomials, giving us a function that is guaranteed to pass through all data points, because for $x_n$ every polynomial, except $W_n$ has a root at $x_n$.
```math
W(x) = W(x_1) + W(x_2) + ... + W(x_n)
```
