pub fn get_neighbors(pt: (i64, i64)) -> Vec<(i64, i64)> {
    vec![
        (pt.0 - 1, pt.1 - 1),
        (pt.0, pt.1 - 1),
        (pt.0 + 1, pt.1 - 1),
        (pt.0 - 1, pt.1),
        (pt.0, pt.1),
        (pt.0 + 1, pt.1),
        (pt.0 - 1, pt.1 + 1),
        (pt.0, pt.1 + 1),
        (pt.0 + 1, pt.1 + 1),
    ]
}

#[derive(Copy, Clone, Debug)]
pub struct SizedBoard<T, const R: usize, const C: usize> {
    pub squares: [[T; C]; R],
}

impl<T, const R: usize, const C: usize> SizedBoard<T, R, C>
where
    T: Clone,
{
    pub fn get_neighbor_spaces(&self, pt: (usize, usize)) -> Vec<T> {
        let nidxs = get_neighbors((pt.0 as i64, pt.1 as i64));
        nidxs
            .iter()
            .map(|pt| self.squares[pt.0 as usize][pt.1 as usize].clone())
            .collect()
    }
}
