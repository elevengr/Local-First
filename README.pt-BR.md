# Local First

[Read in English](README.md)

## Sobre o projeto

O Local First e um rastreador de habitos feito em linguagem Rust que funciona totalmente pelo terminal. O objetivo e ajudar voce a criar, organizar e acompanhar seus habitos diarios de forma simples e rapida, sem precisar de internet ou conta em nuvem.

Todo o projeto gira em torno de um unico arquivo chamado habit.json, que e criado e atualizado automaticamente sempre que voce interage com o programa. Suas informacoes ficam salvas inteiramente no seu computador, garantindo privacidade e controle total sobre seus dados.

O programa oferece seis funcionalidades basicas.

1. Criar um novo habit, informando apenas o nome.
2. Editar o nome de um habit ja cadastrado.
3. Deletar um habit existente.
4. Listar todos os habitos salvos no momento.
5. Marcar um habit como concluido.
6. Sair do programa.

Cada habit possui um identificador unico gerado automaticamente pelo sistema UUID, um nome definido por voce e um status booleano que indica se o habit foi concluido ou nao.


## Como instalar

Para rodar o Local First na sua maquina, voce precisa ter o Rust instalado. Caso ainda nao tenha, acesse o site oficial em https://www.rust-lang.org/tools/install e siga as instrucoes do instalador para o seu sistema operacional.

Com o Rust instalado, abra o terminal e execute os seguintes passos.

Primeiro, clone o repositorio do projeto.

```
git clone https://github.com/seu-usuario/Local-First.git
```

Em seguida, entre na pasta do projeto.

```
cd Local-First
```

Agora compile o projeto com o comando de build.

```
cargo build
```

Esse processo vai baixar todas as dependencias e gerar o executavel dentro da pasta target/debug.


## Como rodar

Para iniciar o programa, execute o comando abaixo dentro da pasta do projeto.

```
cargo run
```

O menu principal vai aparecer no terminal com as seis opcoes disponiveis. Basta digitar o numero correspondente a acao que deseja realizar e pressionar enter.

Na primeira execucao, o arquivo habit.json e criado automaticamente na raiz do projeto. Todas as operacoes futuras de criacao, edicao, delecao, listagem e conclusao atualizam esse arquivo diretamente.


## Exemplos de uso

**Criando um habit**

Ao selecionar a opcao 1, o programa pergunta o nome do habit. Voce digita o nome e o habit e salvo imediatamente.

```
Selectd option: 1
Name: 
Estudar Rust
```

Apos essa acao, o habit e registrado no arquivo habit.json com um identificador unico, o nome informado e o status inicial como falso.

**Listando os habitos**

Ao selecionar a opcao 4, todos os habitos salvos sao exibidos no terminal em formato JSON formatado, facilitando a visualizacao.

**Editando um habit**

Ao selecionar a opcao 2, o programa pede o identificador do habit que voce quer alterar e em seguida o novo nome. O registro e atualizado no arquivo.

**Deletando um habit**

Ao selecionar a opcao 3, voce informa o identificador do habit e ele e removido permanentemente da lista.

**Concluindo um habit**

Ao selecionar a opcao 5, voce informa o identificador do habit e o status dele e alterado para verdadeiro (concluido). Caso o habit nao exista, uma mensagem de erro e exibida.

**Saindo do programa**

Ao selecionar a opcao 6, o programa e encerrado.


## Como o projeto funciona por dentro

O programa e escrito em uma unica fonte de codigo, localizada em src/main.rs. Ele utiliza a biblioteca tokio como runtime assincrono, embora todas as operacoes de entrada e saida sejam executadas de forma sincrona no terminal.

Os dados sao persistidos no formato JSON. A cada operacao, o programa le o conteudo do arquivo habit.json, carrega a lista de habitos em memoria, realiza a acao solicitada e salva a lista atualizada de volta no arquivo.

A estrutura de cada habit contem tres campos: um UUID v4 como identificador unico, uma string com o nome do habit e um valor booleano para o status. A serializacao e desserializacao do JSON sao feitas com as bibliotecas serde e serde_json.


## Dependencias do projeto

O projeto utiliza as seguintes bibliotecas Rust.

1. tokio, para o runtime assincrono.
2. uuid, para geracao de identificadores unicos com suporte a versao 4 e serializacao JSON.
3. serde, para a derivacao automatica de serializacao e desserializacao.
4. serde_json, para leitura e escrita no formato JSON.


## Observacoes importantes

O arquivo habit.json e o unico arquivo de persistencia do projeto e nao deve ser modificado manualmente, pois alteracoes diretas podem comprometer a integridade dos dados.

O projeto esta em fase inicial de desenvolvimento e utiliza a edicao 2024 do Rust. Contribuicoes e melhorias sao bem vindas.
