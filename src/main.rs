use std::io;

fn main() {
    let mut idade_em_dias = String::new();
    io::stdin()
        .read_line(&mut idade_em_dias)
        .expect("Idade não informada");

    let idade_em_dias: u32 = idade_em_dias.trim().parse().expect("Idade inválida");

    let anos = idade_em_dias / 365;
    let meses = (idade_em_dias % 365) / 30;
    let dias = (idade_em_dias % 365) % 30;

    println!("{} ano(s)", anos);
    println!("{} mes(es)", meses);
    println!("{} dia(s)", dias);
}
