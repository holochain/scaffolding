import 'babel-polyfill';
import express from 'express';
import dotenv from 'dotenv';
import bodyParser from 'body-parser';
import open from 'open';
import http from 'http';
import { Server } from 'socket.io';
import fs from 'fs';

import { ClientEventType } from '@holochain-scaffolding/events';
import { writeDirectoryTree } from '@source-craft/fs';
import { automaticSetup } from './events/automatic-setup';

export function launchApp() {
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
    maxHttpBufferSize: 1e8,
  });

  io.on('connection', socket => {
    socket.on(ClientEventType.WriteDirectory, ({ happ, happName }) => {
      const dir = `${process.cwd()}/${happName}`;
      fs.mkdirSync(dir);

      writeDirectoryTree(dir, happ);
    });
    socket.on(ClientEventType.ReadDir, callback => callback({ dirPath: process.cwd() }));
    socket.on(ClientEventType.AutomaticSetup, appName => automaticSetup(appName));
    socket.on(ClientEventType.Exit, () => process.exit());
  });

  console.log(`
                                                          
                                        .                           
                                    ::.:-.-:.-.    .    .           
                               ....-:=-==++***+--==.: :.:..         
                           .  :::--=**%@%@@@@@@@@@@%*-. =:=         
                            . .-.:*%@@@@@@%####%@@@@@@%%+=:...       
                        . :-::*#@@@@#=:           .-*@@@@%*= :: .     
                        -.:-*#@@@@+.                  =@@@@#+-:.  .   
                      .:-::+%@@@#.                      *@@@#-=:. .   
                       :..=*@@@#                         *@@%#-.-:    
                      ..:.#+@@@.                          @@%#-:*:    
                      .::-**@@%                           #@@*=--..   
                     ..:.:=#@@%                           #@@%+:....  
                      :. :+*@@@:                         .@@@%=::..   
                     ::- -+*@@@%.                        #@@@=-::::.  
                      . -=:=*%@@%:                     :%@@%+:.:.     
                       .- .-.+%%@@*:                 :*@@%**.: .-     
                           .:=++%@@@%*-:.       ..-+%@@@#*=+-:  .     
                          .. ...-.-#@@@@@@@@@@@@@@@@@@#+--.::.:      
                              . .::-==-#%%@@@@@@%@@#+--:. ::         
                              . .:-  .:+:=+++***++--:.:-:...         
                                  :  .::-:..::.::: .. ..             
                                        .: . :. .:      .            

  _   _       _            _           _        ______  ___ ______   _____           _     
 | | | |     | |          | |         (_)       | ___ \\/ _ \\|  _  \\ |_   _|         | |    
 | |_| | ___ | | ___   ___| |__   __ _ _ _ __   | |_/ / /_\\ \\ | | |   | | ___   ___ | |___ 
 |  _  |/ _ \\| |/ _ \\ / __| '_ \\ / _\` | | '_ \\  |    /|  _  | | | |   | |/ _ \\ / _ \\| / __|
 | | | | (_) | | (_) | (__| | | | (_| | | | | | | |\\ \\| | | | |/ /    | | (_) | (_) | \\__ \\
 \\_| |_/\\___/|_|\\___/ \\___|_| |_|\\__,_|_|_| |_| \\_| \\_\\_| |_/___/     \\_/\\___/ \\___/|_|___/
                                                                                                              
                                                                                                              
`);

  console.log('');
  console.log('Welcome to the Holochain RAD Tools!');
  console.log('');

  console.log('Go to the browser tab that just opened and scaffold your first Holochain app!');
  console.log('');

  // opens the url in the default browser
  open(URL);
}
