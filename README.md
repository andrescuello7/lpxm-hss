# SSH-MPLX

Un servidor SSH simple implementado en Rust que permite ejecutar comandos remotos a través de conexiones TCP.

## Descripción

SSH-MPLX es un servidor que escucha en el puerto 22 (puerto estándar SSH) y permite a los clientes conectarse y ejecutar comandos del sistema. El servidor procesa los comandos recibidos y devuelve la salida al cliente.

## Características

- Servidor TCP asíncrono usando Tokio
- Manejo de múltiples conexiones concurrentes
- Ejecución segura de comandos del sistema
- Parsing de comandos con soporte para argumentos entre comillas
- Respuesta en tiempo real de la salida de comandos

## Requisitos

- Rust 1.70+ (edición 2024)
- Node.js (para el cliente de ejemplo)

## Instalación

### Compilar el servidor

```bash
cargo build --release
```

El binario se generará en `target/release/ssh-mplx`

### Ejecutar el servidor

```bash
cargo run
```

O si ya está compilado:

```bash
./target/release/ssh-mplx
```

El servidor iniciará y escuchará en `0.0.0.0:22`

**Nota:** Para ejecutar en el puerto 22, necesitarás permisos de administrador (sudo en Linux/Mac).

## Uso

### Servidor

El servidor se ejecuta automáticamente al iniciar y escucha conexiones en el puerto 22. Cada conexión se maneja en un hilo separado, permitiendo múltiples clientes simultáneos.

### Cliente JavaScript

Puedes usar el cliente de ejemplo incluido en `server/main.js` o crear tu propio cliente. Ver la sección de [Ejemplos](#ejemplos) para más detalles.

## Ejemplos

### Cliente JavaScript básico

El cliente básico se encuentra en `server/main.js`:

```bash
node server/main.js
```

### Cliente JavaScript mejorado

Para un ejemplo más completo con mejor manejo de errores, interfaz interactiva y características adicionales, usa:

```bash
node examples/client-example.js
```

Este cliente incluye:
- Interfaz de línea de comandos interactiva
- Manejo robusto de errores
- Indicadores visuales de conexión
- Comando `exit` para cerrar la conexión
- Mejor formato de salida

## Estructura del Proyecto

```
ssh-mplx/
├── Cargo.toml          # Configuración del proyecto Rust
├── src/
│   ├── main.rs         # Punto de entrada del servidor
│   ├── lib.rs          # Biblioteca principal
│   ├── ssh_server/     # Módulo del servidor SSH
│   │   ├── mod.rs
│   │   └── server.rs   # Implementación del servidor
│   └── ssh_key/        # Módulo de manejo de claves SSH
├── server/
│   └── main.js         # Cliente básico en JavaScript
└── README.md           # Este archivo
```

## Seguridad

⚠️ **ADVERTENCIA:** Este es un servidor SSH simplificado para propósitos educativos. No incluye:

- Autenticación de usuarios
- Cifrado de datos
- Validación de comandos
- Protección contra inyección de comandos

**NO debe usarse en producción** sin implementar medidas de seguridad apropiadas.

## Dependencias

- `tokio`: Runtime asíncrono para Rust
- `bytes`: Utilidades para trabajar con bytes
- `thiserror`: Manejo de errores
- `tracing`: Logging y trazabilidad
- `shlex`: Parsing seguro de comandos con shell

## Desarrollo

### Ejecutar tests

```bash
cargo test
```

### Compilar en modo debug

```bash
cargo build
```

## Licencia

[Especificar licencia aquí]

## Contribuciones

Las contribuciones son bienvenidas. Por favor, abre un issue o pull request para cualquier mejora.

