# Manual de usuario — Restaurant POS

Guía completa para operar el sistema en un restaurante.

## Índice

1. [Primer arranque](#1-primer-arranque)
2. [Configurar el restaurante](#2-configurar-el-restaurante)
3. [Crear productos](#3-crear-productos)
4. [Crear mesas](#4-crear-mesas)
5. [Usuarios y roles](#5-usuarios-y-roles)
6. [Operación diaria](#6-operación-diaria)
7. [Cobrar una cuenta](#7-cobrar-una-cuenta)
8. [Cocina (KDS)](#8-cocina-kds)
9. [Reportes](#9-reportes)
10. [Respaldos](#10-respaldos)
11. [Impresión](#11-impresión)
12. [Preguntas frecuentes](#12-preguntas-frecuentes)

---

## 1. Primer arranque

Al abrir la aplicación por primera vez, se inicia en **modo libre**: no
pide credenciales. Puedes empezar a configurar todo de inmediato.

En cuanto crees tu primer usuario, la app pasará a pedir login cada vez
que se abra.

---

## 2. Configurar el restaurante

Ve a **Configuración** (menú superior).

### Idioma

Elige entre **English** y **Español**.

### Información del restaurante

1. Click en **Editar**.
2. Llena:
   - **Nombre** (obligatorio).
   - **Dirección** (opcional).
   - **Teléfono** (opcional).
   - **Mensaje al pie** (opcional; aparece al final de cada recibo).
3. Guarda.

### Carpeta de recibos

Por defecto: `~/Documents/restaurant-receipts`.

Puedes cambiarla con **Cambiar carpeta** o abrirla con **Abrir carpeta**.

---

## 3. Crear productos

Ve a **Productos**.

1. Primero crea al menos una **categoría** (ej: Bebidas, Platos fuertes).
   Click en **+ Agregar categoría**.
2. Luego crea productos dentro de esa categoría.
   Click en **+ Agregar producto**.
3. Llena: nombre, categoría, precio (en tu moneda local), descripción.

**Reglas:**
- No puede haber dos categorías con el mismo nombre.
- No puede haber dos productos con el mismo nombre dentro de la misma categoría.
- El precio debe ser mayor que cero.

---

## 4. Crear mesas

Ve a **Mesas**.

Click en **+ Agregar mesa**. Llena nombre (ej: "Mesa 1"), zona (opcional,
ej: "Terraza"), y capacidad.

**Estados de una mesa:**
- **Libre** (verde): lista para sentar clientes.
- **Ocupada** (rojo): tiene una orden activa.
- **Reservada** (amarillo): reservada para un cliente.
- **Limpieza** (azul): el cliente pagó, falta limpiar.

---

## 5. Usuarios y roles

Ve a **Configuración → Usuarios** (solo admin).

### Roles

| Rol | Acceso |
|---|---|
| **Administrador** | Todas las vistas y configuraciones. |
| **Mesero** | Mesas, TPV, Órdenes, Cocina. |
| **Cocina** | Solo Cocina. |

### Crear un usuario

1. Click en **+ Agregar usuario**.
2. Llena: nombre, PIN (4-8 dígitos), rol.
3. Guarda.

**Regla de seguridad:** no puedes dejar el sistema sin al menos un
administrador activo.

---

## 6. Operación diaria

### Atender una mesa

1. Ve a **TPV**.
2. Selecciona la mesa en el menú desplegable.
3. Toca los productos para agregarlos al carrito.
4. Ajusta cantidades con **+ / − / ×**.
5. Cuando esté lista, click en **Enviar a cocina**.

La mesa pasa automáticamente a estado **Ocupada**.

### Ver órdenes activas

Ve a **Órdenes**. Aparecen todas las órdenes abiertas con:
- Número de orden.
- Hora de creación.
- Mesa asociada.
- Número de ítems.
- Estado.
- Total.

Click en cualquier orden para ver el detalle completo.

### Estados de orden

| Estado | Significado |
|---|---|
| **Abierta** | Creada, aún no enviada a cocina. |
| **En cocina** | Enviada, preparándose. |
| **Servida** | Entregada al cliente. |
| **Pagada** | Cobrada. |
| **Cancelada** | Cancelada. |

---

## 7. Cobrar una cuenta

1. Desde **Órdenes** o **Mesas**, abre el detalle de la orden (debe estar **Servida**).
2. Click en **Cobrar**.
3. Se abre el modal de pago:
   - Elige método: **Efectivo**, **Tarjeta** u **Otro**.
   - El monto ya viene con el restante.
   - Si el cliente paga con más (efectivo), verás el cambio en verde.
4. Click en **Registrar pago**.

**Pagos parciales:** si el cliente paga una parte, puedes registrar varias
rondas hasta completar el total. El modal se cierra automáticamente
cuando el saldo llega a cero.

**Al completar el pago:**
- La orden pasa a **Pagada**.
- La mesa pasa a **Limpieza**.
- Se te pregunta si quieres imprimir el recibo.

---

## 8. Cocina (KDS)

La vista **Cocina** muestra todas las órdenes pendientes con:
- Número de orden.
- Mesa.
- Tiempo transcurrido (color según urgencia).
- Lista de ítems con cantidades.

**Códigos de color:**
- **Verde:** menos de 5 minutos.
- **Amarillo:** entre 5 y 15 minutos.
- **Rojo:** más de 15 minutos (parpadea).

**Acciones:**
- **Imprimir:** genera el ticket de cocina en PDF.
- **Marcar como servida:** la orden sale de la vista.

Las órdenes más antiguas aparecen primero.

---

## 9. Reportes

Ve a **Reportes**.

### Filtros
- **Hoy** (por defecto).
- **Últimos 7 días**.
- **Este mes**.
- **Personalizado:** elige fechas con el calendario.

### Métricas
- **Ventas totales:** suma de pagos recibidos.
- **Órdenes:** número de órdenes únicas cobradas.
- **Ticket promedio:** ventas / órdenes.
- **Pagos:** número total de pagos (incluye parciales).

### Tablas
- **Productos más vendidos:** ordenados por cantidad.
- **Ventas por día:** dentro del rango seleccionado.

---

## 10. Respaldos

Ve a **Configuración → Respaldos** (solo admin).

### Respaldo automático
Por defecto, la app crea **un respaldo por día** al iniciar. Actívalo o
desactívalo con el toggle.

### Respaldo manual
Click en **Crear respaldo ahora**.

### Restaurar
1. Click en **Restaurar** en el respaldo deseado.
2. Acepta la advertencia (los datos actuales serán reemplazados).
3. La app se recarga con los datos del respaldo.

**Importante:** restaurar reemplaza TODO. Crea un respaldo del estado
actual antes si tienes dudas.

### Carpeta de respaldos
Por defecto: `~/.local/share/com.johan.restaurant-app/backups`.

Puedes cambiarla con **Cambiar carpeta**.

---

## 11. Impresión

Cuando cobras una orden, la app ofrece imprimir el **recibo de venta** en
PDF. Desde Cocina, puedes imprimir el **ticket de cocina**.

Los PDFs se guardan en la carpeta configurada y se abren con el visor del
sistema.

**Nota:** actualmente se genera PDF. La impresión directa a impresoras
térmicas ESC/POS estará disponible en una futura versión.

---

## 12. Preguntas frecuentes

### ¿Necesito internet?
No. La app funciona 100 % offline.

### ¿Dónde se guardan mis datos?
En `~/.local/share/com.johan.restaurant-app/restaurant.db`.
Haz respaldos regulares.

### ¿Puedo usar la app en varias cajas?
No en la versión actual. Está planificado para una futura versión.

### ¿Cómo cambio el idioma?
Configuración → Idioma.

### ¿Qué pasa si olvido el PIN del admin?
Contacta al soporte técnico. Se puede resetear con acceso al archivo de base de datos.

### ¿Puedo deshacer un pago?
No directamente. Cancela la orden y créala de nuevo si es necesario.