mod item;
mod mrp;
mod print;

use item::{BomLink, Item, LotSize};

fn main() {
    // ----------------------------------------------------------------
    // Przykładowy scenariusz: produkcja roweru.
    //
    // Struktura BOM (3 poziomy):
    //
    //   Rower (poziom 0)
    //   ├── Rama       x1   (poziom 1)
    //   │   └── Rurka stalowa  x4 (poziom 2)
    //   └── Koło       x2   (poziom 1)
    //       └── Szprycha       x32 (poziom 2)
    // ----------------------------------------------------------------

    let items = vec![
        Item::new("ROWER", "Rower miejski", 0, 2, 5)
            .with_safety_stock(2)
            .with_lot_size(LotSize::LotForLot),
        Item::new("RAMA", "Rama aluminiowa", 1, 3, 4)
            .with_safety_stock(1)
            .with_lot_size(LotSize::Minimum(10))
            .with_scheduled_receipt(1, 8),
        Item::new("KOLO", "Koło 28\"", 1, 2, 10)
            .with_safety_stock(2)
            .with_lot_size(LotSize::FixedBatch(20)),
        Item::new("RURKA", "Rurka stalowa", 2, 1, 30)
            .with_safety_stock(5)
            .with_lot_size(LotSize::FixedBatch(50)),
        Item::new("SZPRYCHA", "Szprycha", 2, 2, 100)
            .with_safety_stock(20)
            .with_lot_size(LotSize::FixedBatch(200)),
    ];

    let boms = vec![
        BomLink {
            parent: "ROWER".into(),
            child: "RAMA".into(),
            qty: 1,
        },
        BomLink {
            parent: "ROWER".into(),
            child: "KOLO".into(),
            qty: 2,
        },
        BomLink {
            parent: "RAMA".into(),
            child: "RURKA".into(),
            qty: 4,
        },
        BomLink {
            parent: "KOLO".into(),
            child: "SZPRYCHA".into(),
            qty: 32,
        },
    ];

    // Popyt niezależny na wyrób końcowy: (id, okres, ilość).
    // Okresy numerowane są od 0 - w wydruku pokazujemy je jako 1..N.
    let demand = vec![
        ("ROWER".to_string(), 4, 10),
        ("ROWER".to_string(), 6, 8),
        ("ROWER".to_string(), 8, 15),
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
