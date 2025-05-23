🧮 Calculadora em Rust
Uma calculadora simples desenvolvida em Rust, com suporte para operações básicas, exponenciação, raiz quadrada e histórico de cálculos.

✨ Funcionalidades
✅ Soma

✅ Subtração

✅ Multiplicação

✅ Divisão com tratamento de erro para divisão por zero

✅ Potenciação

✅ Raiz quadrada com verificação para números negativos

📈 Histórico das operações realizadas

📦 Estrutura do Projeto
enum Operacao: Enumeração das operações suportadas.

enum ErroCalculadora: Enumeração para tratamento de erros (divisão por zero, raiz de número negativo, entrada inválida).

struct Calculadora: Estrutura principal que mantém o histórico e executa os cálculos.

🧠 Como funciona
A função calcular executa a operação desejada com os operandos fornecidos e armazena o resultado em um histórico. Também realiza validações para garantir a integridade dos cálculos.

Exemplo de uso (em um main.rs ou em testes):

rust
Copy
Edit
fn main() {
    let mut calc = Calculadora::nova();

    match calc.calcular(Operacao::Somar, 10.0, Some(5.0)) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("Erro: {:?}", e),
    }

    match calc.calcular(Operacao::RaizQuadrada, 16.0, None) {
        Ok(resultado) => println!("Resultado: {}", resultado),
        Err(e) => println!("Erro: {:?}", e),
    }

    calc.mostrar_historico();
}
🚫 Tratamento de Erros
Divisão por zero: impede que a operação continue.

Raiz quadrada de número negativo: impede a operação e retorna erro.

Entradas inválidas: retorna um erro explicando o que está faltando (por exemplo, segundo número ausente).

📚 Requisitos
Rust (instale via rustup)

🚀 Rodando o Projeto
Clone este repositório:

bash
Copy
Edit
git clone https://github.com/seu-usuario/calculadora-rust.git
cd calculadora-rust
Compile e execute:

bash
Copy
Edit
cargo run
