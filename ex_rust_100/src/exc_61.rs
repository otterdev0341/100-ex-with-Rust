/*
Write a  program that prints all the letters in the alphabet using a loop (one letter per line).

The program should print the letters in uppercase.

Expected Output:
A
B
C
D
E
F
G
H
I
J
K
L
M
N
O
P
Q
R
S
T
U
V
W
X
Y
Z
*/

fn print_A_to_Z() {
    for x in 'A'..='Z' {
        println!("{}",x);
    }
}

#[cfg(test)]
mod test {
    use super::print_A_to_Z;


    #[test]
    fn test_1() {
        print_A_to_Z();
    }
}