
pub fn quick_sort(a: &mut Vec<i32>, p: i32, r: i32){
    // base case
    if p < r {
        // if we are still going through the list
        let q = part(a, p, r);
        // sort a section and return the pivot
        quick_sort(a, p, q-1);
        // take the lower half
        quick_sort(a, q+1, r);
        // take the upper half
    }
    return {};
}

pub fn part(a: &mut Vec<i32>, p: i32, r: i32) -> i32{
    let mut i = p-1;
    for j in p..r{
        if a[j as usize] <= a[r as usize] {
            // there is an out of place index
            i += 1;
            // exchange a[i] and a[j]
            let temp = a[i as usize];
            a[i as usize] = a[j as usize];
            a[j as usize] = temp;
        }
    }
    // move our pivot to the middle and return it as q
    let temp = a[(i+1) as usize];
    a[(i+1) as usize] = a[r as usize];
    a[r as usize] = temp;
    return i+1;
}
