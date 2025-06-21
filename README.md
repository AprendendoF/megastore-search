# Sistema de Busca Otimizado para Catálogo de Produtos - MegaStore

## 📋 Descrição do Projeto
Este projeto implementa um sistema de busca eficiente e escalável para o catálogo da MegaStore, usando a linguagem de programação Rust. O sistema permite buscas rápidas por nome, marca e categoria de produtos, mesmo com milhões de itens.

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
```

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
- `main.rs`: Entrada principal
- `product.rs`: Estrutura de dados de produtos
- `search.rs`: Módulo com lógica de busca
- `lib.rs`: Interface para integração

## 📚 Algoritmos e Estruturas de Dados
- **HashMap**: para indexar e buscar produtos de forma eficiente por múltiplos critérios.
- Técnicas de normalização de texto para melhorar precisão das buscas.

## 📈 Desempenho e Escalabilidade
- Busca em tempo constante (`O(1)` médio).
- Estrutura preparada para particionar dados se necessário (sharding).
- Compatível com cache de buscas.

## 👥 Contribuições
Sinta-se à vontade para abrir issues e pull requests.

## 📄 Licença
MIT
