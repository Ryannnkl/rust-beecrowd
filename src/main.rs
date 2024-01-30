use std::io;

fn read_pedido() -> (f64, f64) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler a entrada");

    let mut iter = input.trim().split_whitespace();
    iter.next(); // Ignorar o primeiro elemento

    let quantidade = iter
        .next()
        .expect("Quantidade não fornecida")
        .parse::<f64>()
        .expect("Erro ao converter quantidade");
    let valor = iter
        .next()
        .expect("Valor não fornecido")
        .parse::<f64>()
        .expect("Erro ao converter valor");

    (quantidade, valor)
}

fn main() {
    let (pedido1_quantidade, pedido1_valor) = read_pedido();
    let (pedido2_quantidade, pedido2_valor) = read_pedido();

    let total = (pedido1_quantidade * pedido1_valor) + (pedido2_quantidade * pedido2_valor);

    println!("VALOR A PAGAR: R$ {:.2}", total);
}
