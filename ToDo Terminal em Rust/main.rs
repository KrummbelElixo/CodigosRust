use std::io;
struct Tarefa {
    objetivo: String,
    concluido: bool,
}

impl Tarefa {
    fn new(objetivo: String, concluido: bool) -> Self {
        Tarefa {
            objetivo,
            concluido
        }
    }

    fn visualizar(&self) {
        println!("[{}] {}", if self.concluido {"X"} else {" "}, &self.objetivo);
    }
}

fn main() {
    let mut tarefas: Vec<Tarefa> = vec![];
    tarefas.push(Tarefa::new("Teste".to_string(), true));
    loop {
        println!("====================");
        println!("To Do App");
        for t in &tarefas {
            Tarefa::visualizar(&t);
        }
        println!("[1] - Concluir/Desconcluir Tarefa");
        println!("[2] - Adicionar tarefa");
        println!("[3] - Remover tarefa");
        println!("====================");
        let escolha: i32 = ler_i32();

        match escolha {
            1 => {
                println!("Informe a tarefa a ser concluida!");
                let concluir: i32 = ler_i32();
                for i in 0..tarefas.len() {
                    if i == (concluir as usize - 1) {
                        if tarefas[i].concluido {
                            tarefas[i].concluido = false;
                        } else {
                            tarefas[i].concluido = true;
                        }
                    }
                }
            }
            2 => {
                tarefas = adicionar_tarefa(tarefas);
            }
            3 => {
                tarefas = remover_tarefa(tarefas);
            }
            _ => {}
        }
    }
}

fn ler_i32() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    return input.trim().parse().unwrap();
}

fn adicionar_tarefa(mut tarefas: Vec<Tarefa>) -> Vec<Tarefa> {
    println!("Informe o objetivo da nova tarefa:");
    let mut objetivo = String::new();
    io::stdin().read_line(&mut objetivo).unwrap();
    tarefas.push(Tarefa::new(objetivo, false));

    return tarefas;
}

fn remover_tarefa(mut tarefas: Vec<Tarefa>) -> Vec<Tarefa> {
    println!("Informe o indice da tarefa a ser removida:");
    let indice: usize = ler_i32() as usize;

    if indice > tarefas.len() || indice <= 0 {
        println!("Informe uma opção válida!");
        return remover_tarefa(tarefas);
    }

    tarefas.remove(indice - 1);

    return tarefas;
}
