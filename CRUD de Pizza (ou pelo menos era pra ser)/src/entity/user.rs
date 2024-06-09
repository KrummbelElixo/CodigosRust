pub mod user {
    use core::fmt;

    use crate::entity::RuleSet;

    struct User {
        nome: String,
        saldo: f32,
        tipo: TipoConta
    }

    impl User {
        fn new(nome: String, saldo: f32, tipo: TipoConta) -> Self {
            User {
                nome,
                saldo,
                tipo
            }
        }
    }

    impl RuleSet for User {
        fn print_struct(&self) {
            println!("nome: {} saldo: {} tipo: {}", self.nome, self.saldo, self.tipo);
        }
    }

    #[derive(Debug)]
    enum TipoConta {
        Admin,
        Cliente,
    }

    impl fmt::Display for TipoConta {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}