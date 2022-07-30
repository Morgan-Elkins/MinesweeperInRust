use rand:: {thread_rng,Rng};

pub fn random_range(min: usize, max: usize) -> usize{
    let mut rng = thread_rng();

    //exclusive range
    let n: usize = rng.gen_range(min..max);
    return n;
}

