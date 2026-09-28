use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use printpdf::*;

use crate::domain::{Order, OrderItem, RestaurantInfo};
use crate::error::{AppError, AppResult};

// Receipt dimensions in millimeters.
// 80mm thermal paper.
const PAGE_WIDTH_MM: f32 = 80.0;
const PAGE_HEIGHT_MM: f32 = 200.0;
const MARGIN_MM: f32 = 5.0;
const LINE_HEIGHT_MM: f32 = 4.5;
const FONT_SIZE: f32 = 9.0;

/// Generate a sales receipt PDF for a paid order.
pub fn generate_receipt(
    order: &Order,
    restaurant: &RestaurantInfo,
    output_path: &Path,
) -> AppResult<()> {
    let (doc, page, layer) = PdfDocument::new(
        "Receipt",
        Mm(PAGE_WIDTH_MM),
        Mm(PAGE_HEIGHT_MM),
        "Receipt",
    );
    let layer = doc.get_page(page).get_layer(layer);
    let font = doc
        .add_builtin_font(BuiltinFont::Courier)
        .map_err(|e| AppError::Internal(format!("font error: {e}")))?;

    let mut y = PAGE_HEIGHT_MM - MARGIN_MM - LINE_HEIGHT_MM;

    // Header: restaurant name centered, address/phone below.
    let name = restaurant.name.to_uppercase();
    write_centered(&layer, &font, &name, &mut y, 12.0, true);
    if let Some(addr) = &restaurant.address {
        write_centered(&layer, &font, addr, &mut y, FONT_SIZE, false);
    }
    if let Some(phone) = &restaurant.phone {
        write_centered(&layer, &font, phone, &mut y, FONT_SIZE, false);
    }

    write_separator(&layer, &font, &mut y);

    // Order header
    write_line(
        &layer,
        &font,
        &format!("Order #{}", order.id),
        &mut y,
        FONT_SIZE,
        true,
    );
    if let Some(table_id) = order.table_id {
        write_line(
            &layer,
            &font,
            &format!("Table: {}", table_id),
            &mut y,
            FONT_SIZE,
            false,
        );
    }
    write_line(
        &layer,
        &font,
        &format!("Date: {}", order.created_at),
        &mut y,
        FONT_SIZE,
        false,
    );

    write_separator(&layer, &font, &mut y);

    // Items
    for item in &order.items {
        if item.status == crate::domain::OrderItemStatus::Cancelled {
            continue;
        }
        write_item_line(&layer, &font, item, &mut y);
        if let Some(notes) = &item.notes {
            write_line(
                &layer,
                &font,
                &format!("   * {}", notes),
                &mut y,
                FONT_SIZE - 1.0,
                false,
            );
        }
    }

    write_separator(&layer, &font, &mut y);

    // Totals
    write_amount_line(
        &layer,
        &font,
        "TOTAL",
        order.total_cents,
        &mut y,
        FONT_SIZE + 1.0,
        true,
    );

    if let Some(msg) = &restaurant.footer_message {
        write_blank(&mut y);
        write_centered(&layer, &font, msg, &mut y, FONT_SIZE, false);
    }

    write_blank(&mut y);
    write_centered(&layer, &font, "Thank you!", &mut y, FONT_SIZE, false);

    // Save
    let file = File::create(output_path)
        .map_err(|e| AppError::Internal(format!("cannot create receipt file: {e}")))?;
    doc.save(&mut BufWriter::new(file))
        .map_err(|e| AppError::Internal(format!("cannot save PDF: {e}")))?;

    Ok(())
}

/// Generate a kitchen ticket PDF for an order.
/// Bold and larger font, no prices, focused on preparation.
pub fn generate_kitchen_ticket(
    order: &Order,
    output_path: &Path,
) -> AppResult<()> {
    let (doc, page, layer) = PdfDocument::new(
        "Kitchen",
        Mm(PAGE_WIDTH_MM),
        Mm(PAGE_HEIGHT_MM),
        "Kitchen",
    );
    let layer = doc.get_page(page).get_layer(layer);
    let font = doc
        .add_builtin_font(BuiltinFont::CourierBold)
        .map_err(|e| AppError::Internal(format!("font error: {e}")))?;

    let mut y = PAGE_HEIGHT_MM - MARGIN_MM - LINE_HEIGHT_MM;

    write_centered(&layer, &font, "KITCHEN", &mut y, 14.0, true);
    write_blank(&mut y);

    write_line(
        &layer,
        &font,
        &format!("ORDER #{}", order.id),
        &mut y,
        14.0,
        true,
    );
    if let Some(table_id) = order.table_id {
        write_line(
            &layer,
            &font,
            &format!("TABLE {}", table_id),
            &mut y,
            12.0,
            true,
        );
    }
    write_line(
        &layer,
        &font,
        &format!("TIME {}", order.created_at),
        &mut y,
        FONT_SIZE,
        false,
    );

    write_separator(&layer, &font, &mut y);

    for item in &order.items {
        if item.status == crate::domain::OrderItemStatus::Cancelled {
            continue;
        }
        write_line(
            &layer,
            &font,
            &format!("{}x  {}", item.quantity, item.product_name),
            &mut y,
            12.0,
            true,
        );
        if let Some(notes) = &item.notes {
            write_line(
                &layer,
                &font,
                &format!("    >> {}", notes),
                &mut y,
                11.0,
                false,
            );
        }
    }

    let file = File::create(output_path)
        .map_err(|e| AppError::Internal(format!("cannot create kitchen ticket: {e}")))?;
    doc.save(&mut BufWriter::new(file))
        .map_err(|e| AppError::Internal(format!("cannot save PDF: {e}")))?;

    Ok(())
}

// ---------- Low-level helpers ----------

fn write_line(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    text: &str,
    y: &mut f32,
    size: f32,
    bold: bool,
) {
    let _ = bold; // Courier vs CourierBold is chosen by font selection
    layer.use_text(text, size, Mm(MARGIN_MM), Mm(*y), font);
    *y -= LINE_HEIGHT_MM;
}

fn write_centered(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    text: &str,
    y: &mut f32,
    size: f32,
    bold: bool,
) {
    // Rough centering: assume ~1.8mm per character at 9pt.
    let char_width = size * 0.18;
    let text_width = text.chars().count() as f32 * char_width;
    let x = (PAGE_WIDTH_MM - text_width) / 2.0;
    let x = x.max(MARGIN_MM);
    let _ = bold;
    layer.use_text(text, size, Mm(x), Mm(*y), font);
    *y -= LINE_HEIGHT_MM;
}

fn write_separator(layer: &PdfLayerReference, font: &IndirectFontRef, y: &mut f32) {
    let line = "-".repeat(38);
    layer.use_text(&line, FONT_SIZE, Mm(MARGIN_MM), Mm(*y), font);
    *y -= LINE_HEIGHT_MM;
}

fn write_blank(y: &mut f32) {
    *y -= LINE_HEIGHT_MM;
}

fn write_item_line(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    item: &OrderItem,
    y: &mut f32,
) {
    let qty_label = format!("{}x {}", item.quantity, item.product_name);
    let amount = format!("{:.2}", (item.unit_price_cents * item.quantity as i64) as f64 / 100.0);

    // Left column: qty + name. Right column: amount.
    // Assume ~38 chars fit in 70mm at 9pt.
    let total_width = 36;
    let left_len = qty_label.chars().count();
    let right_len = amount.chars().count();
    let padding = if left_len + right_len < total_width {
        total_width - left_len - right_len
    } else {
        1
    };
    let line = format!("{}{}{}", qty_label, " ".repeat(padding), amount);

    layer.use_text(&line, FONT_SIZE, Mm(MARGIN_MM), Mm(*y), font);
    *y -= LINE_HEIGHT_MM;
}

fn write_amount_line(
    layer: &PdfLayerReference,
    font: &IndirectFontRef,
    label: &str,
    amount_cents: i64,
    y: &mut f32,
    size: f32,
    bold: bool,
) {
    let amount = format!("{:.2}", amount_cents as f64 / 100.0);
    let total_width = 36;
    let left_len = label.chars().count();
    let right_len = amount.chars().count();
    let padding = if left_len + right_len < total_width {
        total_width - left_len - right_len
    } else {
        1
    };
    let line = format!("{}{}{}", label, " ".repeat(padding), amount);
    let _ = bold;
    layer.use_text(&line, size, Mm(MARGIN_MM), Mm(*y), font);
    *y -= LINE_HEIGHT_MM;
}