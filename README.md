# portfolio_data_structure

# Descrição Projeto
A função principal deste projeto é realizar a aplicação de um modelo de Regressão Linear Simples (ou Pura). Recebendo dados inseridos pelo usuário, calculando a equação da reta que se ajusta melhor aos dados inseridos, realizando também previsões e calculando a métrica de desempenho do modelo.

1- Funcionalidades:
    - Entrada do usuário com 5 dados para o eixo X;
    - Entrada do usuário com 5 dados para o eixo Y;
    - Cálculo dos coeficientes de regressão linear:
        - Inclinação (a);
        - Intercepção (b).
    - Geração da equação modelo y = ax + b;
    - Previsão dos valores de Y a partir dos valores de X;
    - Cálculo das métricas de avaliação do modelo:
        - R² (Coeficiente de Determinação);
        - MSE (Erro Quadrático Médio).
    - Tratamento de erros para divisão por zero;
    - Testes automatizados, garantindo:
        - Correção da Regressão Linear;
        - Precisão das previsões;
        - Validação das métricas (R² e MSE);
        - Verificação de erro em casos especificos.

2- Objetivos do Projeto:
    - Com o objetivo de demonstrar o funcionamento da regressão linear de forma prática e interativa, utilizando conceitos matemáticos e boas práticas de programação em rust. Trazendo a mente o conceito de machine learning, estatistica e rust.

3- Descrição das funções implementadas:
    3.1- regressao_linear(x: &[f64], y: &[f64]) -> (f64, f64)
        Funcionalidade:
        - Realizar a regressão linear simples, com o objetivo de encontrar a equação da reta que melhor se ajusta aos dados fornecidos.
        3.1.1- Entrada:
            - x: Vetor de valores do eixo X;
            - y: Vetor de valores do eixo Y.
        3.1.2- Saída:
            Retorno uma tupla (a,b):
            - a: Coeficiente angular (inclinação da reta);
            - b: Coeficiente linear (interceptação da reta).

    3.2- prever(a: f64, b: f64, x: &[f64]) -> Vec<f64>
        Funcionalidade:
        - Utilizar os coeficientes a e b da regressão para fazer previsões de y com base nos valores de x.
        3.2.1- Entrada:
            - a: Inclinação da reta;
            - b: Interceptação da reta;
            - x: Vetor com valores de X.
        3.2.2- Saída:
            - Vetor com os valores previstos de Y.

    3.3- calcular_metricas(y_real: &[f64], y_pred: &[f64]) -> (f64, f64)
        Funcionalidade:
        - Calcular métricas de avaliação do modelo.
        3.3.1- Entrada:
            - y_real: Valores reais observados;
            - y_pred: Valores previstos pelo modelo.
        3.3.2- Saída:
            Retorno uma tupla (r2, mse):
            - r2: Coeficiente de Determinação (R²) - mede o quão bem o modelo explica os dados;
            - mse: Erro Quadrático Médio - mede o erro médio das previsões.

    3.4- main()
        3.4.1- Funcionalidade:
        - Receber 5 valores do usuário para x;
        - Receber 5 valores do usuário para y;
        - Realizar a regressão linear;
        - Mostrar o modelo obtido (equação da reta);
        - Mostrar as previsões;
        - Calcular e exibir as métricas R² e MSE.

    3.5- Testes Unitários
        Os testes validam as funções implementadas:
        3.5.1- Teste (test_regressao_linear):
            3.5.1.1- Objetivo:
            - Verifica se a função gera corretamente a inclinação e o intercepto.
        3.5.2- Teste (test_prever):
            3.5.2.1- Objetivo:
            - Verifica se a previsão retorna os valores esperados.
        3.5.3- Teste (test_metricas):
            3.5.3.1- Objetivo:
            - Verifica se R² é 1.0 e MSE é 0.0 (perfeito).
        3.5.4- Teste (test_panic_regressao_linear):
            3.5.4.1- Objetivo:
            - Verifica se o código trata erro de divisão por zero.