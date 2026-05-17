use crate::item::Item;
use crate::mrp::MrpPlan;
use std::collections::HashMap;

pub fn print_plan(plan: &MrpPlan, items: &[Item]) {
    let by_id: HashMap<String, &Item> = items.iter().map(|i| (i.id.clone(), i)).collect();

    println!();
    println!("============================================================");
    println!("              PLAN MRP - wynik obliczeń");
    println!("============================================================");
    println!("Horyzont planowania: {} okresów (np. dni)", plan.periods);
    println!();

    for rec in &plan.records {
        let item = by_id.get(&rec.item_id).expect("item musi istnieć");
        print_record(item, rec);
        println!();
    }
}

fn print_record(item: &Item, rec: &crate::mrp::MrpRecord) {
    println!("------------------------------------------------------------");
    println!(
        "[{}] {}   (poziom {}, lead time: {}, stan pocz.: {}, SS: {})",
        item.id, item.name, item.level, item.lead_time, item.on_hand, item.safety_stock
    );
    println!("Reguła partii: {:?}", item.lot_size);
    println!("------------------------------------------------------------");

    let n = rec.periods as usize;
    let col_w = 6;
    let label_w = 28;

    print!("{:label_w$}", "Okres:", label_w = label_w);
    for p in 0..n {
        print!("{:>w$}", p + 1, w = col_w);
    }
    println!();

    print_row(
        label_w,
        col_w,
        "Zapotrzebowanie brutto",
        &rec.gross_requirements,
    );
    print_row(label_w, col_w, "Planowane dostawy", &rec.scheduled_receipts);
    print_row_i(label_w, col_w, "Przewidywany stan", &rec.projected_on_hand);
    print_row(
        label_w,
        col_w,
        "Zapotrzebowanie netto",
        &rec.net_requirements,
    );
    print_row(
        label_w,
        col_w,
        "Plan. przyjęcia zleceń",
        &rec.planned_order_receipts,
    );
    print_row(
        label_w,
        col_w,
        "Plan. wydania zleceń",
        &rec.planned_order_releases,
    );

    let lt = item.lead_time as usize;
    let mut late = false;
    for p in 0..lt.min(n) {
        if rec.net_requirements[p] > 0 {
            late = true;
            break;
        }
    }
    if late {
        println!(
            "  UWAGA: zapotrzebowanie netto pojawia się w okresie < lead time ({}).",
            item.lead_time
        );
        println!("         Zlecenia trzeba było wydać przed startem horyzontu.");
    }
}

fn print_row(label_w: usize, col_w: usize, label: &str, row: &[u32]) {
    print!("{:label_w$}", label, label_w = label_w);
    for v in row {
        if *v == 0 {
            print!("{:>w$}", ".", w = col_w);
        } else {
            print!("{:>w$}", v, w = col_w);
        }
    }
    println!();
}

fn print_row_i(label_w: usize, col_w: usize, label: &str, row: &[i64]) {
    print!("{:label_w$}", label, label_w = label_w);
    for v in row {
        print!("{:>w$}", v, w = col_w);
    }
    println!();
}
