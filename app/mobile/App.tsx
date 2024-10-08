import React from 'react';
import { NavigationContainer, DarkTheme } from '@react-navigation/native';
import { createStackNavigator } from '@react-navigation/stack';
import Events from './src/screens/events';
import { StatusBar } from 'expo-status-bar';
import { StyleSheet, View } from 'react-native';

const Stack = createStackNavigator();

const CustomDarkTheme = {
  ...DarkTheme,
  colors: {
    ...DarkTheme.colors,
    background: '#121212', 
  },
};

export default function App() {
  return (
    <NavigationContainer theme={CustomDarkTheme}>
      <StatusBar style="light" />
      <View style={styles.container}>
        <Stack.Navigator
          screenOptions={{
            cardStyle: { backgroundColor: '#121212' },
          }}
        >
          <Stack.Screen name="Events" component={Events} />
        </Stack.Navigator>
      </View>
    </NavigationContainer>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#121212',
  },
});
