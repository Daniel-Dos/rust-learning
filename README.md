# Rust Learning - CRUD de Usuários

Projeto de aprendizado em Rust implementando um CRUD completo de usuários com SQLite.

## 📋 Descrição

Aplicação assíncrona em Rust que demonstra operações básicas de CRUD (Create, Read, Update, Delete) com banco de dados SQLite, utilizando SQLx para persistência de dados e Tokio como runtime assíncrono.

## 🚀 Tecnologias Utilizadas

- **Rust** (Edition 2024)
- **SQLx** - Driver assíncrono para SQLite
- **Tokio** - Runtime assíncrono
- **Tracing** - Sistema de logging estruturado
- **Anyhow** - Tratamento de erros simplificado
- **Rand** - Geração de dados aleatórios

## 🏗️ Arquitetura

O projeto segue uma arquitetura em camadas:

```mermaid
graph TB
    subgraph "Camada de Apresentação"
        A[main.rs]
        style A fill:#4CAF50,stroke:#2E7D32,stroke-width:3px,color:#fff
    end

    subgraph "Camada de Serviço"
        B[UserService]
        style B fill:#2196F3,stroke:#1565C0,stroke-width:3px,color:#fff
    end

    subgraph "Camada de Repositório"
        C[UserDBSqlite]
        style C fill:#FF9800,stroke:#E65100,stroke-width:3px,color:#fff
    end

    subgraph "Camada de Dados"
        D[(SQLite Database)]
        style D fill:#9C27B0,stroke:#6A1B9A,stroke-width:3px,color:#fff
    end

    subgraph "Modelos"
        E[User Model]
        style E fill:#00BCD4,stroke:#006064,stroke-width:3px,color:#fff
    end

    subgraph "Utilitários"
        F[Utils]
        style F fill:#607D8B,stroke:#37474F,stroke-width:3px,color:#fff
    end

    A -->|usa| B
    A -->|usa| E
    A -->|usa| F
    B -->|usa| C
    B -->|usa| E
    C -->|persiste| D
    C -->|retorna| E
```

## 📦 Funcionalidades

A aplicação demonstra as seguintes operações:

- ✅ **Create**: Criação de usuários com dados aleatórios
- ✅ **Read**: Listagem de todos os usuários
- ✅ **Update**: Atualização de email do usuário
- ✅ **Delete**: Remoção de usuários

## 🛠️ Instalação e Execução

### Pré-requisitos

- Rust (versão 1.93 ou superior)
- Cargo

### Executar localmente

```bash
# Clonar o repositório
git clone <url-do-repositorio>

# Entrar no diretório
cd app

# Executar a aplicação
cargo run
```

### Executar com Docker

```bash
# Build da imagem
docker build -t rust-learning .

# Executar container
docker run rust-learning
```

## 📊 Estrutura do Banco de Dados

```mermaid
erDiagram
    USERS {
        INTEGER id PK "Auto Increment"
        TEXT username "Nome do usuário"
        TEXT email "Email do usuário"
        INTEGER age "Idade"
    }
```

**Schema SQL:**
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    age INTEGER NOT NULL
);
```

## 📝 Fluxo de Execução

O programa executa automaticamente as seguintes operações:

```mermaid
sequenceDiagram
    participant M as Main
    participant S as UserService
    participant R as Repository
    participant DB as SQLite

    rect rgb(232, 245, 233)
        Note over M: 1. Inicialização
        M->>DB: Conectar ao banco
        DB-->>M: Conexão estabelecida
        M->>S: Criar UserService
    end

    rect rgb(227, 242, 253)
        Note over M,DB: 2. Create - Criar Usuário
        M->>M: Gerar dados aleatórios
        M->>S: create_user(user)
        S->>R: save_user(user)
        R->>DB: INSERT INTO users
        DB-->>R: ✓ Sucesso
        R-->>S: Ok
        S-->>M: ✓ Usuário criado
    end

    rect rgb(255, 243, 224)
        Note over M,DB: 3. Read - Buscar Usuários
        M->>S: get_all_users()
        S->>R: find_all()
        R->>DB: SELECT * FROM users
        DB-->>R: Retornar registros
        R-->>S: Vec<User>
        S-->>M: Lista de usuários
    end

    rect rgb(248, 231, 255)
        Note over M,DB: 4. Update - Atualizar Email
        M->>M: Gerar novo email
        M->>S: update_user_email(id, email)
        S->>R: update_user_email(id, email)
        R->>DB: UPDATE users SET email
        DB-->>R: ✓ Sucesso
        R-->>S: Ok
        S-->>M: ✓ Email atualizado
    end

    rect rgb(255, 235, 238)
        Note over M,DB: 5. Delete - Remover Usuário
        M->>S: delete_user(id)
        S->>R: delete_user(id)
        R->>DB: DELETE FROM users
        DB-->>R: ✓ Sucesso
        R-->>S: Ok
        S-->>M: ✓ Usuário deletado
    end
```

## 🔍 Logs

A aplicação utiliza o sistema de tracing para logs estruturados, fornecendo informações detalhadas sobre:
- Inicialização da aplicação
- Operações de CRUD
- Erros e exceções

## 📄 Licença

Projeto de aprendizado - uso livre para fins educacionais.

