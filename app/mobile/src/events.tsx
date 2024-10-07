import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, View, ScrollView } from 'react-native';
import { useEffect, useState } from 'react';
import { fetchEvents, Event } from '../core/utils/api'; 



export default function Events() {
  const [events, setEvents] = useState<Event[]>([]); 
  const [loading, setLoading] = useState(true);
    // await Font.loadAsync({
    //     'CustomFont': require('../assets/fonts/Blur Medium.otf'), // Your custom font
    // });

  useEffect(() => {
    const loadEvents = async () => {
      try {
        const response = await fetchEvents(); 
        console.log("API response:", response);
        if (response && response.length > 0) {
          setEvents(response);
        } else {
          console.error("No events found");
        }
      } catch (error) {
        console.error("Error loading events:", error);
      } finally {
        setLoading(false);
      }
    };

    loadEvents();
  }, []);

  if (loading) {
    return (
      <View style={styles.container}>
        <Text style={styles.loadingText}>Loading events...</Text>
      </View>
    );
  }

  return (
    <ScrollView style={styles.container}>
    <Text style={styles.header}>Browse Events</Text>
    <StatusBar style="auto" />
    {events.map((event, index) => (
        <View key={index} style={styles.eventCard}>
            <View style={styles.eventDetails}>
                <Text style={styles.eventName}>{event.name}</Text>
                <Text style={styles.eventInfo}>{event.location}</Text>
                <Text style={styles.eventInfo}>Tickets: {event.available} / {event.capacity}</Text>
            </View>
            <Text style={styles.eventPrice}>${event.price}</Text>
        </View>
    ))}
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#000', // Changed background color to dark
  },
  header: {
    fontSize: 24,
    fontWeight: 'bold',
    textAlign: 'center',
    marginVertical: 20,
    color: '#fff', // Changed text color to white
  },
  loadingText: {
    color: '#fff', // Changed text color to white
  },
  eventCard: {
    backgroundColor: '#111', // Changed card background color to dark
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
    color: '#fff', // Changed text color to white
  },
  eventInfo: {
    fontSize: 14,
    marginVertical: 2,
    fontWeight: '400',
    color: '#fff', // Changed text color to white
  },
  eventPrice: {
    fontSize: 18,
    fontWeight: 'bold',
    color: '#0a0', // Changed text color to white
  },
});
