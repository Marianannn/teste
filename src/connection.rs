use mysql::*;

fn main(){
    let url = "mysql://root:@127.0.0.1:3306/cadastro";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    println!("deu certo?\n");
}