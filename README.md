# tricoder

Um simples scanner de portas TCP escrito em Rust.

## Descrição

O `tricoder` recebe um domínio, tenta resolver seu endereço IP e escaneia as portas mais comuns, informando quais estão abertas. O domínio padrão usado no exemplo é `scanme.nmap.org`.

## Como usar

### Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) instalado

### Instalação

Clone o repositório e entre na pasta:

```sh
git clone <url-do-repo>
cd tricoder
```

### Execução

Compile e execute:

```sh
cargo run
```

O resultado será uma lista das portas abertas para o domínio configurado no código.

## Exemplo de uso

```rust
let mut subdomain = Subdomain {
    domain: "scanme.nmap.org".to_string(),
    open_ports: Vec::new(),
};

subdomain = scan_ports(subdomain);

println!("{:?}", subdomain.open_ports);
```

## Estrutura dos arquivos

- `src/main.rs`: ponto de entrada do programa
- `src/port.rs`: lógica de varredura de portas
- `src/model.rs`: definição das structs `Port` e `Subdomain`
- `src/common_ports.rs`: lista das portas mais comuns

## Observações

- O domínio pode ser alterado no arquivo `src/main.rs`.
- O programa utiliza a crate [`rayon`](https://crates.io/crates/rayon) para paralelismo (opcional).
- Use apenas para fins educacionais e em alvos autorizados.

---