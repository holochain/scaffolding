import express from 'express';
import dotenv from 'dotenv';
import bodyParser from 'body-parser';
import open from 'open';

//@ts-ignore
import template from './template.rs.hbs';

dotenv.config();

const app = express();

const PORT = process.env.SERVER_PORT || 3000;
app.set('port', PORT);
app.use(bodyParser.json());
app.use(bodyParser.urlencoded({ extended: true }));


const publicPath = __dirname + '/public/';
app.get('/', function(req, res) {
  res.sendFile(publicPath + 'index.html');
});
app.use(express.static(publicPath));

app.listen(app.get('port'), () => {
  console.log('App is running at http://localhost:%d in %s mode', app.get('port'), app.get('env'));
  console.log('Press CTRL-C to stop\n');
});

// opens the url in the default browser
open(`http://localhost:${PORT}`);