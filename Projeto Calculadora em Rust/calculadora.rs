use std::io;

#[derive(Debug)]
enum Operacao {
    Somar,
    Subtrair,
    Multiplicar,
    Dividir,
    Potencia,
    RaizQuadrada,
}

#[derive(Debug)]
enum ErroCalculadora {
    DivisaoPorZero,
    RaizDeNumeroNegativo,
    EntradaInvalida(String),
}

struct Calculadora {
    historico: Vec<String>,
}

impl Calculadora {
    fn nova() -> Calculadora {
        Calculadora {
            historico: Vec::new(),
        }
    }

    fn calcular(&mut self, operacao: Operacao, a: f64, b: Option<f64>) -> Result<f64, ErroCalculadora> {
        let resultado = match operacao {
            Operacao::Somar => {
                let b = b.ok_or(ErroCalculadora::EntradaInvalida("Segundo número necessário".to_string()))?;
                a + b
            }
            Operacao::Subtrair => {
                let b = b.ok_or(ErroCalculadora::EntradaInvalida("Segundo número necessário".to_string()))?;
                a - b
            }
            Operacao::Multiplicar => {
                let b = b.ok_or(ErroCalculadora::EntradaInvalida("Segundo número necessário".to_string()))?;
                a * b
            }
            Operacao::Dividir => {
                let b = b.ok_or(ErroCalculadora::EntradaInvalida("Segundo número necessário".to_string()))?;
                if b == 0.0 {
                    return Err(ErroCalculadora::DivisaoPorZero);
                }
                a / b
            }
            Operacao::Potencia => {
                let b = b.ok_or(ErroCalculadora::EntradaInvalida("Expoente necessário".to_string()))?;
                a.powf(b)
            }
            Operacao::RaizQuadrada => {
                if a < 0.0 {
                    return Err(ErroCalculadora::RaizDeNumeroNegativo);
                }
                a.sqrt()
            }
        };

        let operacao_str = match operacao {
            Operacao::Somar => format!("{} + {} = {}", a, b.unwrap(), resultado),
            Operacao::Subtrair => format!("{} - {} = {}", a, b.unwrap(), resultado),
            Operacao::Multiplicar => format!("{} × {} = {}", a, b.unwrap(), resultado),
            Operacao::Dividir => format!("{} ÷ {} = {}", a, b.unwrap(), resultado),
            Operacao::Potencia => format!("{}^{} = {}", a, b.unwrap(), resultado),
            Operacao::RaizQuadrada => format!("√{} = {}", a, resultado),
        };

        self.historico.push(operacao_str);
        Ok(resultado)
    }

    fn mostrar_historico(&self) {
        if self.historico.is_empty() {
            println!("Nenhum cálculo realizado ainda.");
        } else {
            println!("\nHistórico de Cálculos:");
            for (i, calculo) in self.historico.iter().enumerate() {
                println!("{}. {}", i + 1, calculo);
            }
        }
    }
}
