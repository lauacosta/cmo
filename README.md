---

<div align = "center">

# Codex
Codex es una herramienta sencilla de terminal para manejar tareas desde la consola. 

**Secciones del README:** [Opciones](#Opciones) - [Descripción](#Descripción) - 
<!-- [Instalación](#Instalación) -->

![Screenshots de Codex](screenshot.png)


</div>

---

## Descripción
El programa nace porque quería tener una lista de tareas siempre a mano. El mismo seguirá en desarrollo siempre y cuando se me vayan ocurriendo mejoras.

Codex está escrito en Rust y se apoya en librerías como:
- [`Clap`](https://docs.rs/clap/latest/clap/).
- [`csv`](https://docs.rs/csv/latest/csv/).
- [`Anyhow`](https://docs.rs/anyhow/latest/anyhow/).
- [`Colored`](https://docs.rs/colored/latest/colored/).
- [`Serde`](https://docs.rs/serde/latest/serde/).

Codex sigue la [especificación XDG a la hora de definir directorios base](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html).

## Opciones
### Comandos
Por el momento Codex viene con tres comandos: **'Add'**, **'Print'**.

- **Add** añadirá una tarea (las cuales pueden tener tres ordenes de prioridad: High, Medium y Low) a un archivo `tasks.csv`, ubicado por defecto en `$XDG_DATA_HOME/codex` si está definido, en caso contrario, en `.local/share/codex`:
``` fish
    # Las opciones High, Medium y Low pueden abreviarse con h, m o l respectivamente.
    codex add -p high "Añadir pruebas a Codex"
    codex add -ph "Comprar pan" 
```
- **Print** mostrará por pantalla, en orden de mayor prioridad a menor, las tareas presentes es `tasks.csv`: 
``` fish
    codex print
```
- **Help** mostrará por pantalla un mensaje de ayuda para cada subcomando.
``` fish
    codex help 
    codex help add 
    codex help print
```

<!-- ## Instalación -->
<!-- ### Cargo -->
<!-- Si ya tenés instalado y configurado Rust, podés instalarlo usando: -->
<!-- ``` -->
<!-- cargo install codex -->
<!-- ``` -->
