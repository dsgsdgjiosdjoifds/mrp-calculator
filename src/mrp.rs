use crate::item::{BomLink, Item, LotSize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MrpRecord {
    pub item_id: String,
    pub periods: u32,
    pub gross_requirements: Vec<u32>,
    pub scheduled_receipts: Vec<u32>,
    pub projected_on_hand: Vec<i64>,
    pub net_requirements: Vec<u32>,
    pub planned_order_receipts: Vec<u32>,
    pub planned_order_releases: Vec<u32>,
}

pub struct MrpPlan {
    pub periods: u32,
    pub records: Vec<MrpRecord>,
}

/// Główna funkcja licząca plan MRP.
///
/// - `items` - lista produktów (każdy ze swoim poziomem BOM, lead time itd.)
/// - `boms` - krawędzie BOM (rodzic -> dziecko, ilość na 1 szt. rodzica)
/// - `demand` - popyt niezależny: (id_produktu, okres, ilość)
/// - `periods` - horyzont planowania (liczba okresów)
pub fn calculate(
    items: &[Item],
    boms: &[BomLink],
    demand: &[(String, u32, u32)],
    periods: u32,
) -> MrpPlan {
    let items_by_id: HashMap<String, Item> =
        items.iter().map(|i| (i.id.clone(), i.clone())).collect();

    let mut gross: HashMap<String, Vec<u32>> = items
        .iter()
        .map(|i| (i.id.clone(), vec![0; periods as usize]))
        .collect();

    for (id, period, qty) in demand {
        if let Some(row) = gross.get_mut(id) {
            if (*period as usize) < row.len() {
                row[*period as usize] += qty;
            }
        }
    }

    let mut order: Vec<&Item> = items.iter().collect();
    order.sort_by_key(|i| i.level);

    let mut records: Vec<MrpRecord> = Vec::new();

    for item in order {
        let gr = gross
            .get(&item.id)
            .cloned()
            .unwrap_or_else(|| vec![0; periods as usize]);
        let rec = compute_record(item, &gr, periods);

        for link in boms.iter().filter(|l| l.parent == item.id) {
            if let Some(child_row) = gross.get_mut(&link.child) {
                for p in 0..periods as usize {
                    child_row[p] += rec.planned_order_releases[p] * link.qty;
                }
            }
        }

        records.push(rec);
    }

    let order_index: HashMap<String, usize> = items
        .iter()
        .enumerate()
        .map(|(i, it)| (it.id.clone(), i))
        .collect();
    records.sort_by_key(|r| order_index.get(&r.item_id).copied().unwrap_or(usize::MAX));

    let _ = items_by_id;

    MrpPlan { periods, records }
}

fn compute_record(item: &Item, gross_req: &[u32], periods: u32) -> MrpRecord {
    let n = periods as usize;
    let mut scheduled = vec![0u32; n];
    for (p, q) in &item.scheduled_receipts {
        if (*p as usize) < n {
            scheduled[*p as usize] += q;
        }
    }

    let mut projected = vec![0i64; n];
    let mut net = vec![0u32; n];
    let mut receipts = vec![0u32; n];
    let mut releases = vec![0u32; n];

    let lt = item.lead_time as usize;
    let ss = item.safety_stock as i64;

    let mut prev_on_hand: i64 = item.on_hand as i64;

    for p in 0..n {
        let tentative = prev_on_hand + scheduled[p] as i64 - gross_req[p] as i64;

        if tentative < ss {
            let shortage = (ss - tentative) as u32;
            let order_qty = apply_lot_size(shortage, &item.lot_size);
            net[p] = shortage;
            receipts[p] = order_qty;

            if p >= lt {
                releases[p - lt] += order_qty;
            } else {
                releases[0] += order_qty;
            }

            projected[p] = tentative + order_qty as i64;
        } else {
            projected[p] = tentative;
        }

        prev_on_hand = projected[p];
    }

    MrpRecord {
        item_id: item.id.clone(),
        periods,
        gross_requirements: gross_req.to_vec(),
        scheduled_receipts: scheduled,
        projected_on_hand: projected,
        net_requirements: net,
        planned_order_receipts: receipts,
        planned_order_releases: releases,
    }
}

fn apply_lot_size(needed: u32, lot: &LotSize) -> u32 {
    if needed == 0 {
        return 0;
    }

    match lot {
        LotSize::LotForLot => needed,
        LotSize::FixedBatch(b) => {
            let b = (*b).max(1);
            ((needed + b - 1) / b) * b
        }
        LotSize::Minimum(m) => needed.max(*m),
    }
}
