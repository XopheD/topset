use topset::*;

fn main()
{
    let items = vec![4, 5, 8, 3, 2, 1, 4, 7, 9, 8];

    // getting the four greatest integers (repeating allowed)
    items.iter().cloned()
        .topset(4, i32::gt)
        .into_iter()
        .for_each(|x| eprintln!("in the top 4: {}", x));

    // getting the four smallest integers
    // (we just need to reverse the comparison function)
    items.topset(4, i32::lt)
        .into_iter()
        .for_each(|x| eprintln!("in the last 4: {}", x));
}