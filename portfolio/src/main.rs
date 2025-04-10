use std::io;

// Regressão Linear Pura
pub fn regressao_linear(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let soma_x: f64 = x.iter().sum();
    let soma_y: f64 = y.iter().sum();
    let soma_xy: f64 = x.iter().zip(y).map(|(xi, yi)| xi * yi).sum();
    let soma_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let denominador = n * soma_x2 - soma_x * soma_x;

    if denominador == 0.0 {
        panic!("Divisão por zero na regressão linear");
    }

    let a = (n * soma_xy - soma_x * soma_y) / denominador;
    let b = (soma_y - a * soma_x) / n;

    (a, b) // Inclinação, Intercepto
}

// Previsão com o modelo linear
pub fn prever(a: f64, b: f64, x: &[f64]) -> Vec<f64> {
    x.iter().map(|xi| a * xi + b).collect()
}

// Cálculo de R² e MSE
pub fn calcular_metricas(y_real: &[f64], y_pred: &[f64]) -> (f64, f64) {
    let n = y_real.len() as f64;
    let media_y: f64 = y_real.iter().sum::<f64>() / n;

    let ss_res = y_real.iter()
        .zip(y_pred)
        .map(|(r, p)| (r - p).powi(2))
        .sum::<f64>();

    let ss_tot = y_real.iter()
        .map(|r| (r - media_y).powi(2))
        .sum::<f64>();

    let r2 = 1.0 - (ss_res / ss_tot);
    let mse = ss_res / n;

    (r2, mse)
}

fn main() {
    let mut tempo = Vec::new();
    let mut valores = Vec::new();

    let mut entrada = String::new();

    let mut contador: i32 = 5;

    while contador > 0{
        println!("Digite um valor para o eixo x: ");
        io::stdin().read_line(&mut entrada).expect("Erro ao ler");

        let mut numero: f64 = entrada.trim().parse().expect("Digite um número válido");

        tempo.push(numero);
        entrada.clear();
        numero = 0.0;
        contador = contador - 1;
    }

    contador = 5;

    while contador > 0{
        println!("Digite um valor para o eixo y: ");
        io::stdin().read_line(&mut entrada).expect("Erro ao ler");

        let mut numero: f64 = entrada.trim().parse().expect("Digite um número válido");

        valores.push(numero);
        entrada.clear();
        numero = 0.0;
        contador = contador - 1;
    }

    let (a, b) = regressao_linear(&tempo, &valores);

    println!("Modelo: y = {:.4}x + {:.4}", a, b);

    let previsoes = prever(a, b, &tempo);

    println!("Previsões: {:?}", previsoes);

    let (r2, mse) = calcular_metricas(&valores, &previsoes);

    println!("R²: {:.4}", r2);
    println!("MSE: {:.4}", mse);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regressao_linear() {
        let x = vec![1.0, 2.0, 3.0];
        let y = vec![3.0, 5.0, 7.0];
        let (a, b) = regressao_linear(&x, &y);
        assert!((a - 2.0).abs() < 1e-6);
        assert!((b - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_prever() {
        let a = 2.0;
        let b = 1.0;
        let x = vec![1.0, 2.0, 3.0];
        let esperado = vec![3.0, 5.0, 7.0];
        let resultado = prever(a, b, &x);
        assert_eq!(resultado, esperado);
    }

    #[test]
    fn test_metricas() {
        let y_real = vec![3.0, 5.0, 7.0];
        let y_pred = vec![3.0, 5.0, 7.0];
        let (r2, mse) = calcular_metricas(&y_real, &y_pred);
        assert!((r2 - 1.0).abs() < 1e-6);
        assert!((mse - 0.0).abs() < 1e-6);
    }

    #[test]
    #[should_panic(expected = "Divisão por zero na regressão linear")]
    fn test_panic_regressao_linear() {
        let x = vec![1.0, 1.0, 1.0];
        let y = vec![2.0, 2.0, 2.0];
        regressao_linear(&x, &y);
    }
}
