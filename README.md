---

<div align = "center">

# cmo
cmo es una herramienta sencilla de terminal para manejar tareas desde la consola. 

**Secciones del README:** [Opciones](#Opciones) - [Descripción](#Descripción) - 
<!-- [Instalación](#Instalación) -->

![Screenshots de cmo](cmo_screenshot.png)


</div>

---

## Descripción
`cmo` es una abreviación informal de la frase `_casi me olvido_`.

El programa nace porque quería tener una lista de tareas siempre a mano. El mismo seguirá en desarrollo siempre y cuando se me vayan ocurriendo mejoras.

cmo está escrito en Rust y se apoya en librerías como:
- [`Clap`](https://docs.rs/clap/latest/clap/).
- [`csv`](https://docs.rs/csv/latest/csv/).
- [`Anyhow`](https://docs.rs/anyhow/latest/anyhow/).
- [`Colored`](https://docs.rs/colored/latest/colored/).
- [`Serde`](https://docs.rs/serde/latest/serde/).

`cmo` sigue la [especificación XDG a la hora de definir directorios base](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html).

## Opciones
### Comandos
Por el momento `cmo` viene con tres comandos: **'add'**, **'list'**, **'flip'**, **'remove'** y **'help'**.

- **add** añadirá una tarea (las cuales pueden tener tres ordenes de prioridad: High, Medium y Low) a un archivo `tasks.csv`:
``` bash
    # Las opciones High, Medium y Low pueden abreviarse con h, m o l respectivamente.
    cmo add -p medium "Comprar pan"
    cmo add -ph "Leer El Camino de los Reyes"
```
- **list** mostrará por pantalla, en orden de mayor prioridad a menor, las tareas presentes es `tasks.csv`. También puede elegirse determinadas prioridades: 
``` bash
    cmo list
    cmo list -p medium 
    cmo list -ph
```
- **flip** permite cambiar el estado de una tarea de incompleta a completa, indicando el número de la tarea en el listado: 
``` bash
    cmo flip 10
```

- **remove** permite borrar una tarea, indicando el número de la tarea en el listado: 
``` bash
    cmo remove 10
```

- **help** mostrará por pantalla un mensaje de ayuda para cada sub-comando.
``` bash
    cmo help 
    cmo help add 
    cmo help print
    cmo help flip 
    cmo help remove
```

<!-- ## Instalación -->
<!-- ### Cargo -->
<!-- Si ya tenés instalado y configurado Rust, podés instalarlo usando: -->
<!-- ``` -->
<!-- cargo install cmo -->
<!-- ``` -->
