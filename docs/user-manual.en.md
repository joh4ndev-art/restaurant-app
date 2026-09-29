# User Manual — Restaurant POS

Complete guide to operate the system in a restaurant.

## Table of contents

1. [First launch](#1-first-launch)
2. [Configure the restaurant](#2-configure-the-restaurant)
3. [Create products](#3-create-products)
4. [Create tables](#4-create-tables)
5. [Users and roles](#5-users-and-roles)
6. [Daily operation](#6-daily-operation)
7. [Charging a bill](#7-charging-a-bill)
8. [Kitchen (KDS)](#8-kitchen-kds)
9. [Reports](#9-reports)
10. [Backups](#10-backups)
11. [Printing](#11-printing)
12. [FAQ](#12-faq)

---

## 1. First launch

On first open, the app starts in **free mode**: no login required. You can
start configuring immediately.

As soon as you create your first user, the app will ask for login every
time it opens.

---

## 2. Configure the restaurant

Go to **Settings** (top menu).

### Language
Choose between **English** and **Español**.

### Restaurant information
1. Click **Edit**.
2. Fill in:
   - **Name** (required).
   - **Address** (optional).
   - **Phone** (optional).
   - **Footer message** (optional; appears at the bottom of each receipt).
3. Save.

### Receipt folder
Default: `~/Documents/restaurant-receipts`.

Change it with **Change folder** or open it with **Open folder**.

---

## 3. Create products

Go to **Products**.

1. First create at least one **category** (e.g., Drinks, Main dishes).
   Click **+ Add category**.
2. Then create products inside that category. Click **+ Add product**.
3. Fill in: name, category, price (in your local currency), description.

**Rules:**
- No two categories with the same name.
- No two products with the same name in the same category.
- Price must be greater than zero.

---

## 4. Create tables

Go to **Tables**.

Click **+ Add table**. Fill in name (e.g., "Table 1"), zone (optional,
e.g., "Terrace"), and capacity.

**Table states:**
- **Free** (green): ready for customers.
- **Occupied** (red): has an active order.
- **Reserved** (yellow): reserved for a customer.
- **Cleaning** (blue): customer paid, pending cleaning.

---

## 5. Users and roles

Go to **Settings → Users** (admin only).

### Roles

| Role | Access |
|---|---|
| **Admin** | All views and settings. |
| **Waiter** | Tables, POS, Orders, Kitchen. |
| **Kitchen** | Kitchen only. |

### Create a user
1. Click **+ Add user**.
2. Fill in: name, PIN (4-8 digits), role.
3. Save.

**Security rule:** you cannot leave the system without at least one
active administrator.

---

## 6. Daily operation

### Serving a table
1. Go to **POS**.
2. Select the table from the dropdown.
3. Tap products to add them to the cart.
4. Adjust quantities with **+ / − / ×**.
5. When ready, click **Send to kitchen**.

The table automatically becomes **Occupied**.

### View active orders
Go to **Orders**. All open orders appear with:
- Order number.
- Creation time.
- Associated table.
- Number of items.
- Status.
- Total.

Click any order to see its full detail.

### Order states

| State | Meaning |
|---|---|
| **Open** | Created, not yet sent to kitchen. |
| **In kitchen** | Sent, being prepared. |
| **Served** | Delivered to customer. |
| **Paid** | Charged. |
| **Cancelled** | Cancelled. |

---

## 7. Charging a bill

1. From **Orders** or **Tables**, open the order detail (must be **Served**).
2. Click **Pay**.
3. The payment modal opens:
   - Choose method: **Cash**, **Card**, or **Other**.
   - The amount is pre-filled with the remaining balance.
   - If the customer pays more (cash), you'll see the change in green.
4. Click **Add payment**.

**Partial payments:** if the customer pays part, you can register several
rounds until the total is complete. The modal closes automatically when
the balance reaches zero.

**On full payment:**
- The order becomes **Paid**.
- The table becomes **Cleaning**.
- You are asked whether to print the receipt.

---

## 8. Kitchen (KDS)

The **Kitchen** view shows all pending orders with:
- Order number.
- Table.
- Elapsed time (color by urgency).
- List of items with quantities.

**Color codes:**
- **Green:** less than 5 minutes.
- **Yellow:** between 5 and 15 minutes.
- **Red:** more than 15 minutes (blinks).

**Actions:**
- **Print:** generates the kitchen ticket as PDF.
- **Mark as served:** the order leaves the view.

Oldest orders appear first.

---

## 9. Reports

Go to **Reports**.

### Filters
- **Today** (default).
- **Last 7 days**.
- **This month**.
- **Custom:** pick dates with the calendar.

### Metrics
- **Total sales:** sum of received payments.
- **Orders:** number of unique orders charged.
- **Average ticket:** sales / orders.
- **Payments:** total number of payments (includes partial).

### Tables
- **Top products:** sorted by quantity.
- **Sales by day:** within the selected range.

---

## 10. Backups

Go to **Settings → Backups** (admin only).

### Automatic backup
By default, the app creates **one backup per day** on startup. Toggle on
or off with the switch.

### Manual backup
Click **Create backup now**.

### Restore
1. Click **Restore** on the desired backup.
2. Accept the warning (current data will be replaced).
3. The app reloads with the backup's data.

**Important:** restoring replaces EVERYTHING. Create a backup of the
current state first if in doubt.

### Backup folder
Default: `~/.local/share/com.johan.restaurant-app/backups`.

Change it with **Change folder**.

---

## 11. Printing

When you charge an order, the app offers to print the **sales receipt** as
PDF. From Kitchen, you can print the **kitchen ticket**.

PDFs are saved in the configured folder and opened with the system viewer.

**Note:** currently PDF only. Direct ESC/POS thermal printing will be
available in a future version.

---

## 12. FAQ

### Do I need internet?
No. The app runs 100% offline.

### Where is my data stored?
In `~/.local/share/com.johan.restaurant-app/restaurant.db`.
Make regular backups.

### Can I use the app on multiple registers?
Not in the current version. Planned for a future release.

### How do I change the language?
Settings → Language.

### What if I forget the admin PIN?
Contact technical support. It can be reset with access to the database file.

### Can I undo a payment?
Not directly. Cancel the order and recreate it if needed.