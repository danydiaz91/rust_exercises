fn main() {
    semantics_1();
    semantics_2();
    semantics_3();
    semantics_4();
}

fn semantics_1() {
    let fill_vec = |vec: &Vec<i32>| -> Vec<i32> {
        let mut vec = vec.clone();

        vec.push(22);
        vec.push(44);
        vec.push(66);

        vec
    };

    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn semantics_2() {
    let fill_vec = |vec: &Vec<i32>| -> Vec<i32> {
        let mut vec = vec.clone();

        vec.push(22);
        vec.push(44);
        vec.push(66);

        vec
    };

    let vec0 = Vec::new();
    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn semantics_3() {
    let fill_vec = |mut vec: Vec<i32>| -> Vec<i32> {
        vec.push(22);
        vec.push(44);
        vec.push(66);

        vec
    };

    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn semantics_4() {
    let fill_vec = || -> Vec<i32> {
        let mut vec = Vec::new();

        vec.push(22);
        vec.push(44);
        vec.push(66);

        vec
    };

    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}