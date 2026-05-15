use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum LotSize {
    /// Lot-for-lot - produkujemy/zamawiamy dokładnie tyle ile trzeba.
    LotForLot,
    /// Partie o stałej wielkości - zaokrąglamy w górę do wielokrotności.
    FixedBatch(u32),
    /// Minimalna wielkość partii - co najmniej tyle, potem dokładnie ile trzeba.
    Minimum(u32),
}

#[derive(Debug, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    /// Poziom w BOM: 0 = wyrób końcowy, im wyżej tym głębiej.
    pub level: u32,
    /// Czas realizacji (produkcji lub dostawy) w okresach.
    pub lead_time: u32,
    /// Stan początkowy magazynu.
    pub on_hand: u32,
    /// Zapas bezpieczeństwa - projektowany stan nie może spaść poniżej.
    pub safety_stock: u32,
    /// Reguła wielkości partii.
    pub lot_size: LotSize,
    /// Zaplanowane wcześniej dostawy (np. już w drodze): okres -> ilość.
    pub scheduled_receipts: HashMap<u32, u32>,
}

impl Item {
    pub fn new(id: &str, name: &str, level: u32, lead_time: u32, on_hand: u32) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            level,
            lead_time,
            on_hand,
            safety_stock: 0,
            lot_size: LotSize::LotForLot,
            scheduled_receipts: HashMap::new(),
        }
    }

    pub fn with_safety_stock(mut self, ss: u32) -> Self {
        self.safety_stock = ss;
        self
    }

    pub fn with_lot_size(mut self, lot: LotSize) -> Self {
        self.lot_size = lot;
        self
    }

    pub fn with_scheduled_receipt(mut self, period: u32, qty: u32) -> Self {
        self.scheduled_receipts.insert(period, qty);
        self
    }
}

/// Zależność BOM: aby zrobić 1 sztukę `parent`, potrzeba `qty` sztuk `child`.
#[derive(Debug, Clone)]
pub struct BomLink {
    pub parent: String,
    pub child: String,
    pub qty: u32,
}
