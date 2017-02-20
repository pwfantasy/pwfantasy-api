'use strict';

const morgan = require('morgan');
const compress = require('compression');

module.exports = function(app) {
    let config = {};

    try {
        config = require('./' + process.env.NODE_ENV);
    } catch(e) {
        console.log("No configuration available for this environment");
    }

    if (config.log) {
        app.use(morgan(config.log.level));
    }

    if (config.compression) {
        app.use(compress());
    }

    if (config.webapi) {
        require('../lib/webapi')(app, config);
    }
};