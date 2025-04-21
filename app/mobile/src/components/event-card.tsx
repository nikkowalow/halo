import React from 'react';
import { Event } from '../types/types';
import { StyleSheet, Text, View, Image } from 'react-native';
import Button from './buttons/button';

interface EventComponentProps {
    event: Event;
    fx: (eventId: number) => void;
}

const EventComponent: React.FC<EventComponentProps> = ({ event, fx }) => {
    return (
        <View style={styles.eventCard}>
            <Image 
                source={{ uri: event.cardImageUrl || 'https://via.placeholder.com/300x200.png?text=Event' }}
                style={styles.eventImage}
                resizeMode="cover"
            />
            <View style={styles.eventDetails}>
                <Text style={styles.eventName}>{event.name}</Text>
                <Text style={styles.eventInfo}>{event.location}</Text>
                <Text style={styles.eventInfo}>Tickets: {event.available} / {event.capacity}</Text>
                <Text style={styles.eventPrice}>${event.price}</Text>
                <Button title="Buy Tickets" onPress={() => fx(event.id)} />
            </View>
        </View>
    );
};

export default EventComponent;


const styles = StyleSheet.create({
  eventCard: {
    backgroundColor: '#111',
    marginVertical: 10,
    marginHorizontal: 20,
    borderRadius: 16,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 5,
    elevation: 3,
    overflow: 'hidden',
  },
  eventImage: {
    width: '100%',
    height: 300,
    borderTopLeftRadius: 16,
    borderTopRightRadius: 16,
  },
  eventDetails: {
    padding: 16,
  },
  eventName: {
    fontSize: 20,
    fontWeight: '800',
    marginBottom: 8,
    color: '#fff',
  },
  eventInfo: {
    fontSize: 14,
    marginBottom: 4,
    fontWeight: '400',
    color: '#fff',
  },
  eventPrice: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#0a0',
    marginTop: 10,
    marginBottom: 10,
  },
});