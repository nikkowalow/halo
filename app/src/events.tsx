import { StatusBar } from 'expo-status-bar';
import { StyleSheet, Text, View } from 'react-native';

export default function Events() {
  return (
    <View style={styles.container}>
      <Text>Browse Events</Text>
      <StatusBar style="auto" />
      <Text>Event 1: Music Concert</Text>
      <Text>Event 2: Art Exhibition</Text>
      <Text>Event 3: Tech Conference</Text>
      <Text>Event 4: Food Festival</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#fff',
    alignItems: 'center',
    justifyContent: 'center',
  },
});
