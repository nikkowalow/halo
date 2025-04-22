import React from 'react';
import { View, Text, StyleSheet, Image, Button } from 'react-native';

const ProfileScreen = () => {
    const handleEditProfile = () => {
        // Handle edit profile action
        console.log('Edit Profile Pressed');
    };

    return (
        <View style={styles.container}>
            <Image
                source={{ uri: 'https://via.placeholder.com/150' }}
                style={styles.profileImage}
            />
            <Text style={styles.name}>John Doe</Text>
            <Text style={styles.email}>johndoe@example.com</Text>
            <Button title="Edit Profile" onPress={handleEditProfile} />
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        flex: 1,
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: '#f5f5f5',
    },
    profileImage: {
        width: 150,
        height: 150,
        borderRadius: 75,
        marginBottom: 20,
    },
    name: {
        fontSize: 24,
        fontWeight: 'bold',
        marginBottom: 10,
    },
    email: {
        fontSize: 16,
        color: 'gray',
        marginBottom: 20,
    },
});

export default ProfileScreen;