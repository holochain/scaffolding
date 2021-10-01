# UI Templates

The UI templates can be found in the `templates` folder. They are zipped at build time and copied over to `packages/client/public`, where they become available to the app in `packages/client`.

These templates should follow these rules:

- Have a `start` script that builds and watches the code from the UI, and open a browser tab with its contents.
  - When running this script, if the environment variable `HC_PORT` is set, then they should replace the call to `AppWebsocket.connect()` to point to that port in `localhost`.
- Have a `build` script that builds the app for production and leaves a `dist` folder, with an `index.html` in it.
- In any of their files, they can specify special variables that will be substituted for **when the app is scaffolded**:
  - `HAPP_SCAFFOLDING{installedAppId}`: the app id.
  - `HAPP_SCAFFOLDING{zomeName}`: the name of the first zome in the app.