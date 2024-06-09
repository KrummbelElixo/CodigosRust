pub mod pizza {
    use core::fmt;
    use crate::entity::RuleSet;
    pub struct Pizza {
        nome: String,
        tipo: TipoPizza,
        valor: f32,
    }

    impl Pizza {
        pub fn new(nome: String, tipo: TipoPizza, valor: f32) -> Self {
            Pizza {
                nome,
                tipo,
                valor,
            }
        }

        pub fn set_nome(&mut self) -> &mut String {
            &mut self.nome
        }

        pub fn set_tipo(&mut self) -> &mut TipoPizza {
            &mut self.tipo
        }

        pub fn set_valor(&mut self) -> &mut f32 {
            &mut self.valor
        }
    }

    impl RuleSet for Pizza {
        fn print_struct(&self) {
            println!("Pizza: {} Tipo: {} Valor: {}", self.nome, self.tipo, self.valor);
        }
    }

    impl fmt::Display for Pizza {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Pizza: {}\n   Tipo: {}\n   Valor: R${}", self.nome, self.tipo, self.valor)
        }
    }

    #[derive(Debug)]
    pub enum TipoPizza {
        Salgada,
        Doce,
        NaoEspecificado
    }

    impl fmt::Display for TipoPizza {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }
}