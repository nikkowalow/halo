import React from 'react';
import { View, Text, StyleSheet, Image, Button } from 'react-native';
import { useAuth } from '../auth/AuthContext';

const ProfileScreen = ({ navigation }: any) => {
  const { user, isLoggedIn, logout } = useAuth();

  const handleLogin = () => navigation.navigate('Login');
  const handleSignup = () => navigation.navigate('Signup');
  const handleLogout = () => logout();

  if (!isLoggedIn) {
    return (
      <View style={styles.container}>
        <Text style={styles.title}>You're not logged in.</Text>
        <Button title="Login" onPress={handleLogin} />
        <Button title="Sign Up" onPress={handleSignup} />
      </View>
    );
  }

  return (
    <View style={styles.container}>
      <Image source={{ uri: 'https://via.placeholder.com/150' }} style={styles.profileImage} />
      <Text style={styles.name}>{user.name}</Text>
      <Text style={styles.email}>{user.email}</Text>
      <Button title="Logout" onPress={handleLogout} />
    </View>
  );
};

const styles = StyleSheet.create({
  container: { flex: 1, justifyContent: 'center', alignItems: 'center' },
  title: { fontSize: 22, marginBottom: 20 },
  profileImage: { width: 150, height: 150, borderRadius: 75, marginBottom: 20 },
  name: { fontSize: 24, fontWeight: 'bold', marginBottom: 10 },
  email: { fontSize: 16, color: 'gray', marginBottom: 20 },
});

export default ProfileScreen;
