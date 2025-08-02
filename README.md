# 🦀 Rustlings

Rustlings é um projeto oficial do Rust que oferece pequenos exercícios interativos para ajudar você a aprender a linguagem Rust na prática. É recomendado usar o Rustlings em paralelo com a leitura do [Livro oficial do Rust](https://doc.rust-lang.org/book/).

## 🚀 Início Rápido

### Instalação

```bash
# Instalar Rustlings
cargo install rustlings

# Inicializar projeto
rustlings init

# Navegar para o diretório
cd rustlings

# Iniciar Rustlings
rustlings
```

### Como Usar

1. Execute `rustlings` para iniciar o modo watch
2. Os exercícios são apresentados sequencialmente
3. Edite os arquivos `.rs` para corrigir os erros
4. Salve o arquivo - o Rustlings detectará automaticamente e verificará sua solução
5. Continue para o próximo exercício quando o atual estiver correto

## 📚 Estrutura dos Exercícios

O projeto contém exercícios organizados por tópicos, seguindo a estrutura do Livro do Rust:

| Exercício | Capítulo do Livro | Descrição |
|-----------|-------------------|-----------|
| **00_intro** | - | Introdução ao Rustlings |
| **01_variables** | §3.1 | Variáveis e mutabilidade |
| **02_functions** | §3.3 | Funções |
| **03_if** | §3.5 | Estruturas condicionais |
| **04_primitive_types** | §3.2, §4.3 | Tipos primitivos |
| **05_vecs** | §8.1 | Vetores |
| **06_move_semantics** | §4.1-2 | Semântica de movimento |
| **07_structs** | §5.1, §5.3 | Estruturas |
| **08_enums** | §6, §18.3 | Enumerações |
| **09_strings** | §8.2 | Strings |
| **10_modules** | §7 | Módulos |
| **11_hashmaps** | §8.3 | HashMap |
| **12_options** | §10.1 | Option |
| **13_error_handling** | §9 | Tratamento de erros |
| **14_generics** | §10 | Genéricos |
| **15_traits** | §10.2 | Traits |
| **16_lifetimes** | §10.3 | Lifetimes |
| **17_tests** | §11.1 | Testes |
| **18_iterators** | §13.2-4 | Iteradores |
| **19_smart_pointers** | §15, §16.3 | Smart pointers |
| **20_threads** | §16.1-3 | Threads |
| **21_macros** | §19.5 | Macros |
| **22_clippy** | §21.4 | Clippy |
| **23_conversions** | - | Conversões de tipos |
| **quizzes** | - | Quizzes para testar conhecimento |

## 🗂️ Estrutura do Projeto

```
rustlang/
├── exercises/          # Exercícios para resolver
│   ├── 00_intro/
│   ├── 01_variables/
│   ├── 02_functions/
│   └── ...
├── solutions/          # Soluções dos exercícios
│   ├── 00_intro/
│   ├── 01_variables/
│   └── ...
├── Cargo.toml         # Configuração do projeto
└── README.md          # Este arquivo
```

## 🎯 Comandos Úteis

```bash
# Executar um exercício específico
cargo run --bin variables1

# Executar a solução de um exercício
cargo run --bin variables1_sol

# Verificar todos os exercícios
rustlings verify

# Obter dica para o exercício atual
rustlings hint

# Pular para o próximo exercício
rustlings next
```

## 📖 Recursos Adicionais

- [Site Oficial do Rustlings](https://rustlings.rust-lang.org/)
- [Repositório no GitHub](https://github.com/rust-lang/rustlings)
- [Livro do Rust](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Documentação do Rust](https://doc.rust-lang.org/)

## 🤝 Contribuindo

Rustlings é um projeto de código aberto. Para contribuir:

1. Visite o [repositório oficial](https://github.com/rust-lang/rustlings)
2. Leia as diretrizes de contribuição
3. Faça um fork e envie um pull request

## 📄 Licença

MIT License

---

**Dica**: Se você está começando com Rust, recomendamos ler o [Livro do Rust](https://doc.rust-lang.org/book/) em paralelo com a resolução dos exercícios do Rustlings para uma experiência de aprendizado mais completa.