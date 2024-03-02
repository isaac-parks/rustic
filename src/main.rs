use rand::Rng;

fn quicksort(a: &mut[usize]) {
    fn swap(arr: &mut[usize], x: usize, y: usize) {
        let s: usize = arr[x];
        arr[x] = arr[y];
        arr[y] = s;
    }

    fn partition(arr: &mut[usize], mut l: usize, mut r: usize) -> usize {
        let pi: usize = l;
        l += 1;
        loop {
            while l <= r && arr[l] <= arr[pi]{
                l += 1;
                continue;
            } 

            while r >= l && arr[r] >= arr[pi]{
                r -= 1;
                continue;
            }

            if l >= r {
                swap(arr, pi, r);
                break;
            }

            swap(arr, l, r);
        }

        r
    }

    fn sort(arr: &mut[usize], l: isize, r: isize) {
        if r - l <  1 {
            return;
        }
        let j: usize = partition(arr, l as usize, r as usize); // Index of "inplace element"
        sort(arr, l, j as isize - 1);
        sort(arr, j as isize + 1, r);

    }

    sort(a, 0, (a.len() - 1) as isize)
}



fn main() {
    let mut rng = rand::thread_rng();
    let mut ray: Vec<usize> = (0..100000000).map(|_| rng.gen_range(0..1000001)).collect();
    for i in 1..50 {
        ray[i] = rng.gen_range(1..=100);
    }
    quicksort(&mut ray[..]);

    // println!("{:?}", ray);  
}