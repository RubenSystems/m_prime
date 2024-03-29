pub fn reward(required_out: &[String], prougram_out: (usize, Vec<String>)) -> isize {
    100 * (-diff(required_out, &prougram_out.1) - 1) * (prougram_out.0 as isize) + 1
}

fn diff(v1: &[String], v2: &[String]) -> isize {
    let mut count = 0;
    let len1 = v1.len();
    let len2 = v2.len();
    let min_len = std::cmp::min(len1, len2);

    for i in 0..min_len {
        if v1[i] != v2[i] {
            count += 1;
        }
    }

    (count + (len1 - min_len) + (len2 - min_len)) as isize
}
