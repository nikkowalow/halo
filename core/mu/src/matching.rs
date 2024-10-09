pub struct BuyTicket {
    pub event_id: i32,
    pub amount: i64,
}

pub async fn buy_ticket(buy_ticket: BuyTicket) -> Result<String, String> {
    println!(
        "Preflighting ticket checkout for event: {}",
        buy_ticket.event_id
    );

    let event = match omicron::event(buy_ticket.event_id).await {
        Ok(e) => e,
        Err(e) => return Err(format!("Failed to fetch event: {}", e)),
    };

    println!("available tickets for event: {:?}", event.available);
    if event.available.unwrap_or(0) < buy_ticket.amount {
        return Err(format!(
            "Insufficient tickets supply. Available: {}, Requested: {}",
            event.available.unwrap_or(0),
            buy_ticket.amount
        ));
    }

    println!(
        "Successfully purchased {} ticket(s) for event {}",
        buy_ticket.amount, buy_ticket.event_id
    );
    Ok(format!(
        "Successfully purchased {} ticket(s) for event {}",
        buy_ticket.amount, buy_ticket.event_id
    ))
}
