mod input; 

fn main() {
    let arr = input::to_grid("
v>>>>>v
12345
^?^
> ? ?^
v?v
6789
>>>> v
^    .<
    ");
    println!("{}", arr[1][1])
}
