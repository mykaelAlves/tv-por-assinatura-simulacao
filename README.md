# Análise da Implementação do Servidor em Rust

O backend da aplicação de simulação de TV por assinatura foi desenvolvido em **Rust**, utilizando um conjunto de bibliotecas modernas do ecossistema para criar uma **API RESTful assíncrona, segura e eficiente**. A seguir, detalhamos os principais componentes da arquitetura.

---

## 1. Arquitetura e Frameworks Base

A aplicação é construída sobre o runtime assíncrono **Tokio** e utiliza o framework web **Axum** para a gestão de rotas e requisições HTTP.

- **Tokio** (`#[tokio::main]`):  
  Serve como o motor assíncrono da aplicação. Ele permite que o servidor execute múltiplas operações de I/O (entrada/saída) de forma concorrente e sem bloqueios, garantindo **alta performance e escalabilidade**.

- **Axum** (`axum::Router`):  
  Utilizado para definir a estrutura da API. Por meio de uma interface declarativa, mapeia os **endpoints da API (URLs)** e os métodos HTTP (GET, POST, PUT, DELETE) às suas respectivas **funções de tratamento (handlers)**.

---

## 2. Gestão de Estado Centralizado e Concorrente

Um dos aspectos centrais da arquitetura é a **gestão do estado da aplicação**, mantido em memória e **compartilhado de forma segura entre requisições concorrentes**.

- **`GlobalState`** (`server/src/lib.rs`):  
  Uma `struct` agregadora que centraliza o estado da aplicação, contendo:
  
  - `planos_disponiveis`: um `Vec` com os planos de assinatura disponíveis.  
  - `assinaturas`: um `HashMap` que associa um `Cliente` ao seu plano correspondente, representando as **assinaturas ativas**.

- **Concorrência Segura com `Arc<Mutex<...>>`**:

  - **`Arc`** (*Atomic Reference Counting*):  
    Permite que múltiplas *threads* compartilhem a mesma instância de `GlobalState`, com contagem de referência automática.

  - **`Mutex`** (*Mutual Exclusion*):  
    Garante que apenas uma *thread* possa modificar o estado por vez. A função `handler` deve adquirir o bloqueio (`.lock().await`) antes de acessar os dados, **evitando condições de corrida** (*race conditions*).

---

## 3. Modelagem de Dados e Serialização com Serde

A comunicação entre cliente (em JSON) e servidor (com tipos Rust) é facilitada pela biblioteca **Serde**.

- **Modelos de Dados** (`server/src/models/`):  
  Os dados de domínio, como `Cliente` e `Plano`, são definidos como `structs` e `enums` em Rust.

- **Atributos `#[derive(Serialize, Deserialize)]`**:  
  Permitem que o compilador gere automaticamente o código para:

  - **Deserialização**:  
    Converter JSON recebido em uma instância de uma `struct` Rust.  
    Exemplo: `Json(cliente): Json<Cliente>`

  - **Serialização**:  
    Converter uma `struct` ou `enum` Rust em JSON para ser enviado como resposta HTTP.

---

## 4. Lógica de Negócio nos Handlers da API (`server/src/services/api.rs`)

Cada endpoint da API é implementado por uma **função handler assíncrona** que contém a lógica de negócio. O Axum utiliza *extratores* para acessar os dados da requisição de forma ergonômica.

- **Extratores Axum**:

  - `State(global_state)`: injeta a referência ao estado global.  
  - `Path(plano_idx)`: extrai parâmetros dinâmicos da URL.  
  - `Json(payload)`: deserializa o corpo da requisição.

- **Fluxo de Execução Típico** de um handler:

  1. Adquire o bloqueio do `Mutex` para acessar o estado global.  
  2. Valida a requisição com base nas regras de negócio.  
     Ex: verificar se um cliente já possui um plano em `assinar_plano`.  
  3. Modifica o estado, se necessário.  
     Ex: inserir ou remover uma entrada no `HashMap` de assinaturas.  
  4. Retorna uma resposta HTTP com:
     - Corpo em JSON  
     - Código de status apropriado (`StatusCode::OK`, `StatusCode::BAD_REQUEST`, etc.)

---
