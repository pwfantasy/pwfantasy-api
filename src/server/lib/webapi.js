'use strict';

const fs = require('fs');
const proxy = require('express-http-proxy');
const path = require('path');
const stubs = path.join(__dirname, '../', 'stubs');

module.exports = function(app, config) {
    if (config.webapi.stubs) {
        app.use('/api', function(req, res, next) {
            let method = req.method.toLowerCase();
            let file = [req.url, method, 'json'].join('.');
            let stub = path.join(stubs, file);

            fs.access(stub, function(err) {
                if (err) return next();

                res.set('X-PWF-Stub', 'true');
                res.sendFile(stub);
            });
        });
    }

    if (config.webapi.proxy) {
        app.use('/api', proxy(config.webapi.proxy.target));
    }
};