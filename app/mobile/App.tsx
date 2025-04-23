import React from 'react';
import { NavigationContainer, DarkTheme } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack'; // switched from stack to native-stack
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import Events from './src/screens/events';
import { AuthProvider } from './src/auth/AuthContext';
import Profile from './src/screens/profile';
import Login from './src/screens/login';
import { StatusBar } from 'expo-status-bar';
import { StyleSheet, View } from 'react-native';

const Stack = createNativeStackNavigator();
const Tab = createBottomTabNavigator();

const CustomDarkTheme = {
  ...DarkTheme,
  colors: {
    ...DarkTheme.colors,
    background: '#121212',
  },
};

function EventsStack() {
  return (
    <Stack.Navigator
      screenOptions={{
        contentStyle: { backgroundColor: '#121212' },
      }}
    >
      <Stack.Screen name="Events" component={Events} />
    </Stack.Navigator>
  );
}

export default function App() {
  return (
    <AuthProvider>
        <NavigationContainer theme={CustomDarkTheme}>
        <StatusBar style="light" />
        <View style={styles.container}>
            <Tab.Navigator
            screenOptions={{
                headerShown: false,
                tabBarStyle: { backgroundColor: '#1e1e1e', borderTopColor: 'transparent' },
                tabBarActiveTintColor: '#fff',
                tabBarInactiveTintColor: '#888',
            }}
            >
            <Tab.Screen name="EventsTab" component={EventsStack} options={{ title: 'Events' }} />
            <Tab.Screen name="Profile" component={Profile} />
            <Tab.Screen name="Login" component={Login} />
            </Tab.Navigator>
        </View>
        </NavigationContainer>
    </AuthProvider>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: '#121212',
  },
});
