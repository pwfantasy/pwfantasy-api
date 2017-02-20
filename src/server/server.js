process.env.NODE_ENV = process.env.NODE_ENV || 'local';

const path = require('path');
const express = require('express');

const app = express();
const PORT = process.env.PORT || 1337;

const root = path.join(__dirname, '../../dist');

app.use(express.static(root));
app.set('view engine', 'html');

require('./config')(app);

// always fallback to index.html
app.use(function(req, res) {
    res.sendFile(path.join(root, 'index.html'));
});

app.listen(PORT, function() {
    console.log('Server running on port: %d', PORT);
    console.log('Running in environment: %s', process.env.NODE_ENV);
});
