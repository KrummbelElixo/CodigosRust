use std::{io, str::FromStr};

mod entity;

use entity::pizza::pizza::*;

fn main() {
    let mut pizzas: Vec<Pizza> = vec![];

    loop {
        println!("Cadastro de Pizzas");
        println!("[1] - Cadastrar pizza");
        println!("[2] - Visualizar pizzas");
        println!("[3] - Modificar cadastro de pizza");
        println!("[4] - Deletar cadastro de pizza");
        let escolha = ler_dados::<i32>().unwrap();

        match escolha {
            1 => {
                pizzas.push(new_pizza());
            }

            2 => {
                for p in &pizzas {
                    println!("{}", p);
                }
            }

            3 => {
                if pizzas.len() == 0 {
                    println!("não há pizzas cadastradas ate o momento");
                    continue;
                }
                
                pizzas = modificar_pizza(pizzas);
            }

            4 => {
                if pizzas.len() == 0 {
                    println!("não há pizzas cadastradas ate o momento");
                    continue;
                }

                for (i, p) in pizzas.iter().enumerate() {
                    println!("Pizza {}\n{}", i, p);
                }

                println!("Informe o indice da pizza a ser removida:");

                let remover = loop {
                    let Some(remover) = ler_dados::<usize>() else {
                        println!("Informe um dado válido!");
                        continue;
                    };

                    if remover >= pizzas.len() {
                        println!("Informe um cadastro existente!");
                        continue;
                    }

                    break remover;
                };

                pizzas.remove(remover);
            }

            _ => println!("Opção inválida ou inexistente!"),
        }
    }
}

fn new_pizza() -> Pizza {
    println!("A pizza será de que?");
    let nome = ler_dados::<String>().unwrap();
    println!("Qual o tipo da Pizza?");
    println!("[1] - Salgada");
    println!("[2] - Doce");
    println!("[Qualquer] - Não especificada");
    let tipo = definir_tipo_pizza(ler_dados::<i32>().unwrap());
    println!("Qual o valor da pizza?");
    let valor = loop {
        let Some(valor) = ler_dados::<f32>() else {
            println!("Informe um dado válido!");
            continue;
        };

        break valor;
    };

    Pizza::new(nome, tipo, valor)
}

fn ler_dados<F : FromStr>() -> Option<F> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<F>().ok()
}

fn definir_tipo_pizza(tipo: i32) -> TipoPizza {
    match tipo {
        1 => TipoPizza::Salgada,
        2 => TipoPizza::Doce,
        _ => TipoPizza::NaoEspecificado
    }
}

fn modificar_pizza(mut pizzas: Vec<Pizza>) -> Vec<Pizza> {
    for (i, p) in pizzas.iter().enumerate() {
        println!("Pizza {}\n{}", i, p);
    }

    println!("Informe o indice da pizza a ser modificada:");

    let alterar = loop {
        let Some(alterar) = ler_dados::<usize>() else {
            println!("Informe um dado válido!");
            continue;
        };

        if alterar >= pizzas.len() {
            println!("Informe um cadastro existente!");
            continue;
        }

        break alterar;
    };

    println!("O que deseja modificar?");
    println!("[1] - Nome");
    println!("[2] - Tipo das pizzas");
    println!("[3] - Valor");

    let escolha = ler_dados::<i32>().unwrap();

    match escolha {
        1 => {
            println!("Informe um novo nome:");
            *pizzas[alterar].set_nome() = ler_dados::<String>().unwrap();
            pizzas
        }
        
        2 => {
            println!("Altere o tipo:");
            println!("[1] - Salgada");
            println!("[2] - Doce");
            println!("[Qualquer] - Não especificada");
            let tipo = ler_dados::<i32>().unwrap();
            *pizzas[alterar].set_tipo() = definir_tipo_pizza(tipo);
            pizzas
        }

        3 => {
            println!("Informe um novo valor:");
            *pizzas[alterar].set_valor() = ler_dados::<f32>().unwrap();
            pizzas
        }

        _ => {
            pizzas
        }
    }
}
