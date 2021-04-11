fn is_op(input: &char) -> bool {
    let op = ['+', '*'];
    let mut flag: bool = false;
    for elem in op.iter() {
        if elem == input {
            flag = true;
            break;
        }
    }
    flag
}

fn main() {
    // polish to reverse polish
    // "+ 1 * 2 3" to "1 2 3 * +" left to right scan "2 3 * 1 +"
    // "* + 1 2 3" to "1 2 + 3 *"
    let input = ['*', '5', '+', '1', '*', '2', '3'];

    let mut i = input.len();
    let mut output = ['0'; 7];
    let mut j = 0;
    let a: usize = 0;

    while i > a {
        i -= 1;
        println!("{}", i);
        if is_op(&input[i]) {
            if j == 0 {
                let arg1 = input[i + 1];
                let arg2 = input[i + 2];

                output[j] = arg1;
                j += 1;

                output[j] = arg2;
                j += 1;

                output[j] = input[i];
                j += 1;
            } else {
                let arg1 = input[i + 1];

                output[j] = arg1;
                j += 1;

                output[j] = input[i];
                j += 1;
            }
        }
    }
    println!("{:?}", output);
}
