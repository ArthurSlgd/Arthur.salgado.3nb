use std::collections::HashMap;

fn main () {
    // Criação do HashMap

    let mut estoque = HashMap::new();

    //1. Inserção de valores
    estoque.insert("banana", 100);
    estoque.insert("pepino", 50);
    estoque.insert("maçã", 2);
    estoque.insert("caqui", 20 );


    //2. Acessar de forma segura os valores do hashmap
    if let Some(qtde) = estoque.get("maçã") {
        println!("Temos {:?} maçãs em estoque! PORRA", qtde);
    }

    //3. atualizando estoque com entry()
    estoque.entry("pepino").and_modify(|qtde| *qtde += 100);
    if let Some(qtde) = estoque.get("pepino") {
        println!("Temos {:?} pepinos em estoque! PORRA", qtde);
    }

    //4. Remover o Caqui
    estoque.remove("caqui");
    println!("{:?}", estoque);

    //5. Filtrar todas as frutas acima de 100 unidades
    estoque.retain(|fruta, &mut qtde| qtde > 100);
    println!("{:?}", estoque);

    //6. Limpeza TOTAL!
    estoque.clear();
    println!("{:?}", estoque)

}