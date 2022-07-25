fn create_buffer(len: usize) -> Vec<u32>{
    vec![0; len]
}

fn fib(len: usize) -> Vec<u32>{
    let mut buff = create_buffer(len);

    let mut old = 0;
    let mut new = 1;

    for i in 0..len{
        buff[i] = old;
        let tmp = old + new;
        old = new;
        new = tmp;
    }

    return buff;
}


fn main() {
    let buff = fib(15);

    for i in buff{
        println!("{i}")
    }

}
