# Window Visibility Toggle in Rust

Este proyecto en Rust permite interactuar con ventanas de aplicaciones en el sistema operativo Windows. La funcionalidad principal es buscar una ventana cuyo título termine con un sufijo específico, alternar su visibilidad y, si está visible, maximizarla y colocarla en primer plano.

## Descripción

Este programa realiza las siguientes operaciones sobre las ventanas de Windows:

1. **Buscar una ventana**: Encuentra la primera ventana cuyo título termine con un sufijo específico.
2. **Alternar visibilidad**: Si la ventana está visible, la oculta; si está oculta, la muestra.
3. **Maximizar y superponer**: Si la ventana se vuelve visible, se maximiza y se coloca en el primer plano, superponiéndola sobre otras ventanas.

## Requisitos

- **Rust**: Se necesita tener Rust instalado. Si no lo tienes, puedes instalarlo siguiendo las instrucciones en [rust-lang.org](https://www.rust-lang.org/).
- **Windows**: Este proyecto está diseñado para ser ejecutado en sistemas operativos Windows.

## Instalación

Para compilar y ejecutar el proyecto, sigue estos pasos:

1. Clona este repositorio:

    ```bash
    git clone https://github.com/tu_usuario/nombre_del_repositorio.git
    cd nombre_del_repositorio
    ```

2. Asegúrate de tener `cargo` y `rustc` instalados:

    ```bash
    rustc --version
    cargo --version
    ```

3. Instala las dependencias y compila el proyecto:

    ```bash
    cargo build --release
    ```

4. Ejecuta el programa:

    ```bash
    cargo run
    ```

## Uso

### Sufijo de Título de la Ventana

El programa busca la primera ventana cuyo título termine con un sufijo específico. En el código de ejemplo, el sufijo utilizado es `"Obsidian v1.7.7"`, pero puedes modificar esta cadena para adaptarla a tus necesidades.

```rust
let title_suffix = "Obsidian v1.7.7"; // Sufijo exacto del título a buscar
