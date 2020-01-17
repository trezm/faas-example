const addon = require('../native');
const port = 3000

console.log(`Example app listening on port ${port}!`);
addon.start(port);
