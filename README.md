````markdown
# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📋 Descrição do Projeto

Este é um projeto pessoal em que desenvolvi um sistema de busca eficiente e escalável para um catálogo de produtos da "MegaStore", utilizando a linguagem de programação Rust. A proposta foi criar um mecanismo capaz de realizar buscas rápidas por nome, marca e categoria — mesmo com uma base de dados grande.

## 🚀 Tecnologias Utilizadas

- Linguagem: **Rust**
- Estrutura de dados: `HashMap`
- Testes: `cargo test`
- Gerenciador de pacotes: `cargo`

## 🛠 Como Executar o Sistema

```bash
git clone https://github.com/seu-usuario/megastore-search.git
cd megastore-search
cargo run
````

## 🧪 Como Executar os Testes

```bash
cargo test
```

## 💡 Exemplos de Uso

```rust
search_by_name("notebook");
search_by_category("eletrônicos");
search_by_brand("Samsung");
```

## 🏗️ Arquitetura do Sistema

* `main.rs`: Ponto de entrada principal do programa.
* `product.rs`: Define a estrutura dos produtos.
* `search.rs`: Contém a lógica de indexação e busca.
* `lib.rs`: Arquivo reservado para futuras integrações com bibliotecas ou APIs.

## 📚 Algoritmos e Estruturas de Dados

* Utilizei um **HashMap** (ou `Vec`, dependendo da versão) para indexar e filtrar os produtos por diferentes critérios de forma eficiente.
* Apliquei técnicas simples de **normalização de texto** (como `to_lowercase`) para tornar a busca mais precisa, mesmo com diferenças de capitalização.

## 📈 Desempenho e Escalabilidade

* A busca funciona em tempo constante médio (`O(1)`) quando feita com `HashMap`.
* O sistema foi pensado para ser facilmente expandido com particionamento (sharding) e cache de buscas, caso seja necessário escalar.

## 👥 Contribuições

Esse projeto foi desenvolvido com fins de aprendizado e demonstração. Fique à vontade para contribuir, clonar, adaptar ou expandir como quiser!

```

---
```
