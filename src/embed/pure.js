'use strict';

const cluster = require('cluster');

if (cluster.isMaster) {
    for (let i = 0; i < 10; i++) {
        let cp = cluster.fork();
        cp.on('message', msg => console.log(msg));
    }
} else {
    let x = 0;
    for (let i = 0; i < 5000000; i++) {
        x += 1;
    }
    process.send(`Thread finished with count=${x}`);
    process.exit(1);
}
