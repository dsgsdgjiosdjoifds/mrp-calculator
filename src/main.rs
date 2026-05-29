mod item;
mod mrp;
mod print;

use item::{BomLink, Item, LotSize};

fn main() {
    let items = vec![
        Item::new("STOL", "Stół drewniany", 0, 2, 3)
            .with_safety_stock(1)
            .with_lot_size(LotSize::LotForLot),
        Item::new("BLAT", "Blat drewniany", 1, 3, 2)
            .with_safety_stock(1)
            .with_lot_size(LotSize::Minimum(10))
            .with_scheduled_receipt(1, 5),
        Item::new("NOGA", "Noga stołowa", 1, 2, 8)
            .with_safety_stock(2)
            .with_lot_size(LotSize::FixedBatch(20)),
        Item::new("DESKA", "Deska dębowa", 2, 2, 20)
            .with_safety_stock(5)
            .with_lot_size(LotSize::FixedBatch(50)),
        Item::new("SRUBA", "Śruba mocująca", 2, 1, 50)
            .with_safety_stock(20)
            .with_lot_size(LotSize::FixedBatch(100)),
    ];

    let boms = vec![
        BomLink {
            parent: "STOL".into(),
            child: "BLAT".into(),
            qty: 1,
        },
        BomLink {
            parent: "STOL".into(),
            child: "NOGA".into(),
            qty: 4,
        },
        BomLink {
            parent: "BLAT".into(),
            child: "DESKA".into(),
            qty: 5,
        },
        BomLink {
            parent: "NOGA".into(),
            child: "SRUBA".into(),
            qty: 4,
        },
    ];

    // Popyt niezależny na wyrób końcowy: (id, okres, ilość).
    // Okresy numerowane są od 0 - w wydruku pokazujemy je jako 1..N.
    let demand = vec![
        ("STOL".to_string(), 4, 8),
        ("STOL".to_string(), 6, 5),
        ("STOL".to_string(), 8, 12),
    ];

    let periods = 10;

    println!("Dane wejściowe:");
    println!("  Produkty: {}", items.len());
    println!("  Zależności BOM: {}", boms.len());
    println!("  Pozycje popytu: {}", demand.len());
    println!("  Horyzont: {} okresów", periods);
    for (id, p, q) in &demand {
        println!("   - popyt: {} szt. produktu {} w okresie {}", q, id, p + 1);
    }

    let plan = mrp::calculate(&items, &boms, &demand, periods);
    print::print_plan(&plan, &items);

    println!("Legenda:");
    println!("  '.'  = 0 (puste pole dla czytelności)");
    println!("  SS   = safety stock (zapas bezpieczeństwa)");
    println!("  Plan. wydania zleceń = kiedy należy rozpocząć produkcję/zamówienie,");
    println!("                         aby przyjęcie nastąpiło po lead time.");
}
