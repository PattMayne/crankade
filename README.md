# CRANKADE

Like and ARCADE for CRANKS.
CRANKADE is the authentication app I use to serve my web apps and games. So if I have multiple apps, a user won't need an account on each one. It provides streamlined authentication services to multiple sites.
I use JSON webtokens (JWTs), reinforced by refresh_tokens. The JWTs are created locally on the client site, the refresh tokens are created on the auth site and stored in a database.


### Client Tokens Structure:
The client apps set JWTs as access tokens into the user's browser's secure cookies. JWTs expire every 20 minutes and are refreshed based on user's refresh token (which is also stored in a secure cookie). JWTs are not stored on any server, only in the browser. But each client app can verify the token, and each client app has its own JWT secret.

The auth app (this app) issues the refresh tokens and saves them in the database. **Problem:** the auth app cannot set cookies for a user who is interacting with a different URL. **Solution:** when the user's refresh_token expires, the client app uses its client_secret to communicate with the auth app (this app), and the auth app issues a new refresh_token. The client app can then set the refresh token (and a new JWT) into the user's browser's secure cookies.

Client sites are stored in a clients table in the DB.
Refresh tokens are stored in a refresh_tokens table in the DB. Refresh token entries include a user_id and a client_id. A user has a different token for each client.

For most requests the user makes on the client site, they do NOT need to directly interact with the auth app (this app). Client apps make a backend call to the auth app's APIs, so no sensitive info ever touches JavaScript.

Email verification is provided through the service "resend".

### RESOURCES FILE
* French and English values are stored in a phf::phf_map!
* * keys are all static string slice references
* Lang field in UserReqData dictates which language version to use
* Each Askama html template has a dedicated struct for all text
