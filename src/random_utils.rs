use rand::Rng;


const CHARSET: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM0123456789";


pub fn randint(min: i32, max: i32) -> i32 {
    // could just use rand::thread_rng().gen_range(min..max); but i wanted to try and implement it
    let mut rng = rand::thread_rng();
    let modulo: u32 = (max - min) as u32;
    min + (rng.gen::<u32>() % modulo) as i32
}


pub fn randstr(length: usize) -> String {
    let mut bound = length;
    if bound > 31 {
        bound = 31;
    }
    let mut rng = rand::thread_rng();
    let mut result: [char; 32] = Default::default();
    for i in 0..bound {
        result[i] = (CHARSET.as_bytes()[rng.gen::<usize>() % (CHARSET.len() - 1)]) as char;
    };
    result[0..bound].iter().collect()
}
