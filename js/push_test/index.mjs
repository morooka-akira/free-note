import admin from 'firebase-admin';
import fs from 'fs';
var serviceAccount = fs.readFileSync("/Users/mor/Downloads/test-sound.json", 'utf-8');
var setting = JSON.parse(serviceAccount)
console.log(setting)

admin.initializeApp({
  credential: admin.credential.cert(serviceAccount)
});
const message = {
  data: {
    score: '850',
    time: '2:45'
  },
  token: "dFy-ZivdSVG6BiHDludNrr:APA91bE9O2HI9XcJBaPTo6dMbL4V5MCtpwqslcFl3Z6lSWqvCzm-3tNG2iIEtQqmEmfuvixOyXwVzx_Y_r2Mf-vNVgoBqHJfCGZDuZJsoVxt3Ex8UqL_w8y-EpEwkA1CVW2f3dB8BTMe"
};

// Send a message to the device corresponding to the provided
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
