import React from 'react';
import { Event } from '../types/types';
import { StyleSheet, Text, View, ScrollView } from 'react-native';
import Button from './buttons/button';
import { buyTicket } from '../services/api';

interface EventComponentProps {
    event: Event;
}

const EventComponent: React.FC<EventComponentProps & { fx: (eventId: number) => void }> = ({ event, fx }) => {

    return (
        <View style={styles.eventCard}>
            <View style={styles.eventDetails}>
                <Text style={styles.eventName}>{event.name}</Text>
                <Text style={styles.eventInfo}>{event.location}</Text>
                <Text style={styles.eventInfo}>Tickets: {event.available} / {event.capacity}</Text>
            </View>
            <View>
                <Text style={styles.eventPrice}>${event.price}</Text>
                <Button title="Buy Tickets" onPress={() => fx(event.id)} />
            </View>
        </View>
    );
};

export default EventComponent;

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#000', 
  },
  header: {
    fontSize: 24,
    fontWeight: 'bold',
    textAlign: 'center',
    marginVertical: 20,
    color: '#fff', 
    fontFamily: 'CustomFont'
  },
  loadingText: {
    color: '#fff',
  },
  eventCard: {
    backgroundColor: '#111', 
    padding: 20,
    marginVertical: 10,
    marginHorizontal: 20,
    borderRadius: 10,
    shadowColor: '#000',
    shadowOffset: { width: 0, height: 2 },
    shadowOpacity: 0.1,
    shadowRadius: 5,
    elevation: 3,
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  eventDetails: {
    flex: 1,
  },
  eventName: {
    fontSize: 20,
    fontWeight: '800',
    marginVertical: 8,
    color: '#fff', 
  },
  eventInfo: {
    fontSize: 14,
    marginVertical: 2,
    fontWeight: '400',
    color: '#fff', 
  },
  eventPrice: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#0a0', 
  },
});
