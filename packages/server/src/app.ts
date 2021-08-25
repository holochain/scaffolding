import express from 'express';
import dotenv from 'dotenv';
import bodyParser from 'body-parser';
import open from 'open';
import http from 'http';
import { Server } from 'socket.io';

import { ClientEvents, ClientEventType, ServerEvents } from '@holochain/create-types';
import { applyGeneratedChanges } from './events/apply-changes';

dotenv.config();

const app = express();
const server = http.createServer(app);

const PORT = process.env.SERVER_PORT || 3000;
const URL = `http://localhost:${PORT}`;

app.set('port', PORT);
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));

const publicPath = __dirname + '/public/';
app.get('/', function(req, res) {
  res.sendFile(publicPath + 'index.html');
});
app.use(express.static(publicPath));

server.listen(app.get('port'), () => {
  console.log('App is running at http://localhost:%d in %s mode', app.get('port'), app.get('env'));
  console.log('Press CTRL-C to stop\n');
});

const io = new Server(server, {
  cors: {
    origin: [URL],
  },
});

io.on('connection', socket => {
  socket.on(ClientEventType.ApplyChanges, changes => applyGeneratedChanges(process.cwd(), changes));
  socket.on(ClientEventType.ReadDir, callback => callback({ dirPath: process.cwd() }));
});

// opens the url in the default browser
open(URL);
