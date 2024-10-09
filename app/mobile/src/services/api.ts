import { Event, Ticket } from "../types/types";
import Config from "react-native-config";

// const API = Config.LOCAL_API_URL;
const API = "http://127.0.0.1:8081";
console.log("API:", API);

export async function fetchEvents(): Promise<Event[]> {
    try {
        const response = await fetch(`${API}/events`, {
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

export async function fetchTickets(eventId: number): Promise<Ticket[]> {
    try {
        const response = await fetch(`${API}/tickets/${eventId}`, {
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

export const buyTicket = async (eventId: number) => {
    try {
        const socket = new WebSocket('ws://127.0.0.1:8080');

        socket.onopen = () => {
            console.log('WebSocket connection established');
            socket.send(JSON.stringify({ action: 'buyTicket', eventId }));
        };

        socket.onerror = (error) => {
            console.error("WebSocket error:", error);
            alert("Error buying event. Please try again.");
        };

        socket.onclose = () => {
            console.log('WebSocket connection closed');
        };
    } catch (error) {
        console.error("Error buying ticket:", error);
        alert("Error buying event. Please try again.");
    }
};


