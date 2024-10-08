export interface Ticket {
    id: number;
    eventId: number;
    holderName: string;
    price: number;
}

export interface Event {
    id: number;
    name: string;
    location: String;
    capacity: number;
    available: number;
    price: number;
}

