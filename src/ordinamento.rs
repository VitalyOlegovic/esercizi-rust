use itertools::Itertools;

pub fn prova() {
    let mut vettore = [1,2,3,0,-1,5,-10];
    vettore.sort_by(|x,y|x.cmp(y).reverse());
    let formatted = vettore.iter().format(" ");

    println!("{}", formatted);
}