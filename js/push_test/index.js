var admin = require("firebase-admin");

var serviceAccount = require("/Users/mor/Downloads/service.json");
console.log(serviceAccount)

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount)
});

const message = {
    "android": {
        "notification": {
            "title": "Breaking hoe",
            "body": "New news story available.",
            "sound": "notification_default2.mp2",
        }
    },
    "token": "<token>"
}

 // registration token.
const res = admin.messaging().send(message)
    .then((response) => {
        // Response is a message ID string.
        console.log('Successfully sent message:', response);
    })
    .catch((error) => {
        console.log('Error sending message:', error);
    });
console.log(res)
