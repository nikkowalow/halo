export interface Event {
    id: number;
    name: string;
    location: String;
    capacity: number;
    available: number;
    price: number;
}

export async function fetchEvents(): Promise<Event[]> {
    try {
        const response = await fetch("http://127.0.0.1:8080/events", {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok) {
            throw new Error(`Failed to fetch events: ${response.status} ${response.statusText}`);
        }

        const events: Event[] = await response.json();
        console.log("API response:", events);

        for (const event of events) {
            const tickets = await fetchTickets(event.id);
            event.available = tickets.length;
            if(tickets.length > 0) {
                event.price = tickets[0].price;
            }
        }


        return events;
    } catch (error) {
        console.error("Error:", error);
        throw error; 
    }
}
export interface Ticket {
    id: number;
    eventId: number;
    holderName: string;
    price: number;
}

export async function fetchTickets(eventId: number): Promise<Ticket[]> {
    try {
        const response = await fetch(`http://127.0.0.1:8080/tickets/${eventId}`, {
            method: "GET",
            headers: {
                "Content-Type": "application/json",
            },
        });

        if (!response.ok) {
            throw new Error(`Failed to fetch tickets: ${response.status} ${response.statusText}`);
        }

        const result: Ticket[] = await response.json();
        console.log("API response:", result);

        return result;
    } catch (error) {
        console.error("Error:", error);
        throw error; 
    }
}
