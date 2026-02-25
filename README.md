# 🦀 Blog-RS

Um blog pessoal feito do zero em Rust, como projeto de aprendizado.

## Tecnologias

- **[Axum](https://github.com/tokio-rs/axum)** — framework web
- **[Tokio](https://tokio.rs)** — runtime assíncrono
- **[Rusqlite](https://github.com/rusqlite/rusqlite)** — banco de dados SQLite
- **[Askama](https://github.com/djc/askama)** — templates HTML
- **[Tower HTTP](https://github.com/tower-rs/tower-http)** — arquivos estáticos

## Estrutura

```
ferrugem/
├── src/
│   ├── main.rs       # ponto de entrada e configuração do servidor
│   ├── db.rs         # conexão e queries do banco de dados
│   ├── models.rs     # estruturas de dados (Post, NewPost)
│   └── routes.rs     # rotas HTTP
├── templates/
│   ├── base.html     # layout base
│   ├── index.html    # listagem de posts
│   ├── post.html     # post individual
│   └── new_post.html # formulário de criação
└── static/
    └── style.css     # estilos (tema jornal, JetBrains Mono)
```

## Como rodar

Certifique-se de ter o [Rust](https://rustup.rs) instalado.

```bash
# clonar o repositório
git clone https://github.com/seu-usuario/ferrugem
cd ferrugem

# compilar e rodar
cargo run
```

O blog estará disponível em `http://localhost:3000`.

## Funcionalidades

- Listagem de posts na página inicial
- Leitura de post individual
- Criação de novos posts via formulário
- Banco de dados SQLite local (gerado automaticamente na primeira execução)

## Roadmap

- [ ] Autenticação — proteger a criação de posts com login
- [ ] Edição e exclusão de posts
- [ ] Suporte a Markdown no conteúdo dos posts
- [ ] Deploy em produção

## Licença

MIT
