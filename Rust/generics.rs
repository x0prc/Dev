// to create arrays of the same size
struct ArrayPair<T, const N: usize> {
    left: [T; N],
    right: [T; N],
}

impl<T: Debug, const N: usize> Debug for ArrayPair<T,N>
fn func<const N: usize>(){}

fn func<const N: usize>() {}

fn bar<T, const M: usize>() {
    func::<M>();
    let _: [u8, M];
}
