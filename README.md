# b_encontrar_todas_as_nfes_do_cte
Este programa busca informações de chaves NFes/CTes em documentos fiscais de formato xml.

Em seguida, retém todas as chaves de NFes vinculadas aos CTes correspondentes no arquivo:

* <cte_nfes.txt>

# Encontrando o Complemento do CTe (Conhecimento de Transporte Eletrônico)

Este projeto Rust fornece uma ferramenta simples para encontrar NFe (Notas Fiscal Eletrônica) vinculada ao CTe (Conhecimento de Transporte Eletrônico).

## O que é um NFe?

A Nota Fiscal Eletrônica (NFe) é um documento que tem como objetivo registrar as operações de compra e vendas de produtos ou serviços. 

## O que é um CTe?

O Conhecimento de Transporte Eletrônico (CTe) é um documento fiscal digital, utilizado no Brasil para acobertar operações de transporte de cargas.

## Como Usar

1.  **Certifique-se de ter o Rust instalado:** Se você ainda não tem o Rust, siga as instruções em [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2.  **Clone o repositório:**
    ```bash
    git clone https://github.com/claudiofsr/b_encontrar_todas_as_nfes_do_cte.git
    cd b_encontrar_todas_as_nfes_do_cte
    ```

3.  **Compile e instale o projeto:**
    ```bash
    cargo build --release && cargo install --path=.
    ```

4.  **Execute o programa:**
    Em um diretório contendo arquivos de CTe em formato XML, execute:
    ```bash
    b_encontrar_todas_as_nfes_do_cte
    ```
