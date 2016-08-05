'use strict';

const path = require('path');
const ffi = require('ffi');

const lib = ffi.Library(path.resolve('./target/release/libembed'), {
    process: ['void', []]
});

lib.process();

console.log('done!');
