use std::any::{Any, TypeId};
use combinations::Combinations;
use std::slice::RChunks;

/*fn generate_lst_combs(lst:Vec<i32>, max_handle:i8) -> Vec<Vec<i8>> {
    let lst_len: i8 = lst.len() as i8;
    let indices: Vec<i8> = (0..lst_len).collect::<Vec<i8>>();
    if max_handle >= lst_len {
        return vec![indices]
    }
    let comb_len: i8 = min(lst_len, max_handle);
    return Combinations::new(
        indices,
        comb_len as usize
    ).collect();
}*/

struct Point {
    x: i8,
    y: i8,
}

impl Point {
    fn distance(self, pnt: Point) -> f64 {
        let delta: Vec<i8> = vec![pnt.x - self.x, pnt.y - self.y];
        let power: f64 = delta.iter().map(|&x| x * x).sum::<i8>() as f64;
        return power.sqrt();
    }
}

fn sequence_to_list<LType: PartialEq>(mut seq: impl Iterator<Item=LType>) -> Vec<LType> {
    return Vec::from_iter(seq.take_while(|x| Some(x) != None));
}

fn chunkify_list(lst: Vec<i8>, max_handle: i8) -> Vec<Vec<i8>> {
    let mut iter = lst.rchunks(max_handle as usize).into_iter();
    let chunked: Vec<_> = sequence_to_list(iter);
    return chunked.iter().map(|a| a.to_owned().to_vec()).collect();
}

fn main() {
    let computed: Vec<Vec<i8>> = chunkify_list(
        (0..100).collect::<Vec<i8>>(),
        3,
    );
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 3, y: 4 };
    println!("{:?}", computed);
    println!("{:?}", p1.distance(p2));
}

#[cfg(test)]
mod wandering_tests {
    use super::*;

    #[test]
    fn chunks_list_properly() {
        assert_eq!(chunkify_list(vec![1, 2, 3, 4], 2).len(), 2 as usize);
    }
}
